use lazy_static::lazy_static;
use rig::{completion::Prompt, providers::anthropic};
use rig::{completion::ToolDefinition, tool::Tool};

struct MockProjectUntitled {
    food_prefs: Vec<StoreFoodPrefArgsAux>,
    diets: Vec<StoreDietTypeArgsAux>,
}

impl MockProjectUntitled {
    pub fn new() -> Self {
        MockProjectUntitled {
            food_prefs: Vec::new(),
            diets: Vec::new(),
        }
    }

    pub async fn add_food_pref(&mut self, food_pref: StoreFoodPrefArgsAux) {
        self.food_prefs.push(food_pref);
    }

    pub async fn add_diet(&mut self, diet: StoreDietTypeArgsAux) {
        self.diets.push(diet);
    }

    pub async fn get_food_prefs(&self) -> &Vec<StoreFoodPrefArgsAux> {
        &self.food_prefs
    }

    pub async fn get_diets(&self) -> &Vec<StoreDietTypeArgsAux> {
        &self.diets
    }
}

lazy_static! {
    static ref GLOBAL_MOCK_PROJECT: tokio::sync::Mutex<MockProjectUntitled> =
        tokio::sync::Mutex::new(MockProjectUntitled::new());
}
#[derive(Debug, serde::Deserialize)]
struct StoreFoodPrefArgsAux {
    #[allow(dead_code)]
    dish: String,
    #[allow(dead_code)]
    kind: String,
}

#[derive(Debug, serde::Deserialize)]
struct StoreFoodPrefArgs {
    dishes: Vec<StoreFoodPrefArgsAux>,
}

#[derive(Debug, serde::Deserialize)]
struct StoreDietTypeArgsAux {
    #[allow(dead_code)]
    diet_type: String,
    #[allow(dead_code)]
    follows: bool,
}

#[derive(Debug, serde::Deserialize)]
struct StoreDietTypeArgs {
    diets: Vec<StoreDietTypeArgsAux>,
}

#[derive(Debug, thiserror::Error)]
#[error("Food pref error")]
struct FoodPrefError;

#[derive(serde::Deserialize, serde::Serialize)]
struct StoreFoodPref;

#[derive(serde::Deserialize, serde::Serialize)]
struct StoreDietType;

impl Tool for StoreFoodPref {
    const NAME: &'static str = "StoreFoodPref";

    type Error = FoodPrefError;
    type Args = StoreFoodPrefArgs;
    type Output = bool;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Store food preferences".to_string(),
            parameters: serde_json::json!(
            {
                "type": "object",
                "properties": {
                    "dishes": {
                        "type": "array",
                        "description": "A list of zero or more dishes and kinds to store",
                        "items": {
                            "type": "object",
                            "properties": {
                                "dish": {
                                    "type": "string",
                                    "description": "The name of the dish or food for which we are learning about a preference"
                                },
                                "kind": {
                                    "type": "string",
                                    "description": "The kind of food preference that this is",
                                    "enum": ["allergy", "intolerance", "dislike", "not_sure", "like", "love"]
                                }
                            },
                            "required": ["dish", "kind"]
                        }
                    }
                },
                "required": ["dishes"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("Storing args {:?}", args);
        let mut project_untitled = GLOBAL_MOCK_PROJECT.lock().await;
        for arg in args.dishes {
            project_untitled.add_food_pref(arg).await;
        }
        Ok(true)
    }
}

impl Tool for StoreDietType {
    const NAME: &'static str = "StoreDietType";

    type Error = FoodPrefError;
    type Args = StoreDietTypeArgs;
    type Output = bool;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Store diet types".to_string(),
            parameters: serde_json::json!(
            {
                "type": "object",
                "properties": {
                    "diets": {
                        "type": "array",
                        "description": "A list of zero or more diet types and whether the user follows them",
                        "items": {
                            "type": "object",
                            "properties": {
                                "diet_type": {
                                    "type": "string",
                                    "description": "The type of diet",
                                    "enum": ["vegetarian", "vegan", "gluten-free", "pescatarian", "keto", "paleo"]
                                },
                                "follows": {
                                    "type": "boolean",
                                    "description": "Whether the user follows this diet"
                                }
                            },
                            "required": ["diet_type", "follows"]
                        }
                    }
                },
                "required": ["diets"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("Storing diet types {:?}", args);
        let mut project_untitled = GLOBAL_MOCK_PROJECT.lock().await;
        for arg in args.diets {
            project_untitled.add_diet(arg).await;
        }

        Ok(true)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = anthropic::Client::from_env();

    let store_food = StoreFoodPref {};
    let store_diet = StoreDietType {};

    let preamble = "Your job is to extract facts about the user's food preferences from snippets of conversation that the user has either with himself or others around him. You will be fed conversation snippets. You do not need to provide any food preferences or diets, or you may learn about more than one at a time.";

    let food_agent = client
        .agent("claude-3-5-sonnet-latest")
        .preamble(preamble)
        .tool(store_food)
        .max_tokens(2048)
        .build();

    let diet_agent = client
        .agent("claude-3-5-sonnet-latest")
        .preamble(preamble)
        .tool(store_diet)
        .max_tokens(2048)
        .build();

    let meal_builder_agent = client
        .agent("claude-3-5-sonnet-latest")
        .preamble("Your job is to generate recipes for meals based on availability of ingredients in your house and the diets and food preferences of the user. After repeating back to the user their preferences and restrictions, please suggest 3 different recipes for dishes to cook that respect the ingredients and the preferences and restrictions.")
        .max_tokens(2048)
        .build();

    let conversation_snippet = "
User: I don't eat pigs or any animals!
Friend: Really? What are you vegetarian.
User: Yeah I'm vegetarian, but not vegan.
User: I love scrambled eggs.
Friend: Really?
User: Yeah they are so good.
User: But I'm allergic to tomatoes.
Friend: How did you learn that?
User: Well I went to the doctor once and he told me.
Friend: Wow it must be hard to be allergic to tomatoes.
User: Yes very hard.
Friend: Do you have any dreams?
User: Yeah of elephants.
        ";

    println!("\n\n\n");
    println!("The following conversation is recorded during everyday life by your little AI device you wear.");
    println!("\n\n");
    println!("{}", &conversation_snippet);
    println!("\n\n\n");

    println!("Now the food agent runs");
    // Send a prompt to the model and await the response.
    let prompt = format!(
        "Here is the conversation to analyze:\n {}",
        conversation_snippet
    );
    let response = food_agent.prompt(&prompt).await?;
    println!("FoodAgent success: {}", response);
    println!("\n\nNow the diet agent runs");
    let response_ = diet_agent.prompt(&prompt).await?;
    println!("DietAgent response: {}", response_);

    println!("\n\n\n");
    println!("Later the chef agent asks project untitled for your diet and food preferences");
    println!("\n\n");

    let project_untitled = GLOBAL_MOCK_PROJECT.lock().await;
    let diets = project_untitled.get_diets().await;
    let food_prefs = project_untitled.get_food_prefs().await;

    let ingredients = "
      Tomatoes
      Pork
      Apples
      Celery
      Bananas
      Salt
      Pepper
      Sancho Pepper
      Miso
      Butter
      Milk
      Eggs
      Flour
      Sugar
      Onions
      Potatoes
      Cilantro
      Chives
      Garlic
      Ginger
      Soy Sauce
      Wasabi";

    println!("\n\n");
    println!("The chef agent knows what ingredients are in your kitchen");
    println!("\n\n");
    println!("Ingredients: {}", &ingredients);
    println!("\n\n");

    println!("Finally the chef agent can combine the info from project untitled about your food preferences and diet with the ingredients to suggest some recipes for you for your meal");
    println!("\n\n");

    let prompt1 = format!(
        "
    Ingredients: {}

    Diets: {:?}

    Food Preferences: {:?}
        ",
        ingredients, diets, food_prefs
    );
    let response__ = meal_builder_agent.prompt(&prompt1).await?;
    println!("MealBuilder agent response: {}", response__);

    println!("\n\n");
    println!("In the future, your array of kitchen robots that all have agents work together to cook one of these meals for you; pulling more personal info about you from project untitled if you give permission.");
    println!("\n\n");
    println!("The End");

    Ok(())
}
