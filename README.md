# Food Project Untitled Concept

This is a little proof of concept demo of what using [Project Untitled] as a shared permissioned state layer for a swarm of AI agents could look like.

Note: For now, [Project Untitled] interactions are mocked behind a simple append-only log in memory, but this can be connected to [Project Untitled] in the future!

## Example Output

The following conversation is recorded during everyday life by your little AI device you wear.




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



Now the food agent runs
Storing args StoreFoodPrefArgs { dishes: [StoreFoodPrefArgsAux { dish: "meat", kind: "dislike" }, StoreFoodPrefArgsAux { dish: "scrambled eggs", kind: "love" }, StoreFoodPrefArgsAux { dish: "tomatoes", kind: "allergy" }] }
FoodAgent success: true


Now the diet agent runs
Storing diet types StoreDietTypeArgs { diets: [StoreDietTypeArgsAux { diet_type: "vegetarian", follows: true }, StoreDietTypeArgsAux { diet_type: "vegan", follows: false }] }
DietAgent response: true




Later the chef agent asks project untitled for your diet and food preferences






The chef agent knows what ingredients are in your kitchen



Ingredients: 
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
      Wasabi



Finally the chef agent can combine the info from project untitled about your food preferences and diet with the ingredients to suggest some recipes for you for your meal



MealBuilder agent response: Let me summarize your dietary requirements and preferences:
- You follow a vegetarian diet
- You are not vegan (so eggs and dairy are okay)
- You dislike meat
- You love scrambled eggs
- You have an allergy to tomatoes
- You have a variety of ingredients including produce, eggs, dairy, and Asian condiments

Based on these parameters, here are 3 recipe suggestions:

1. Fluffy Japanese-Style Scrambled Eggs (Tamago)
- Whisk eggs with a splash of milk, miso, and soy sauce
- Cook slowly in butter for a custardy texture
- Garnish with chives
- Serve with sautéed potatoes and onions on the side

2. Apple-Celery Breakfast Hash
- Diced potatoes sautéed until crispy
- Caramelized onions
- Diced apples and celery for crunch
- Seasoned with salt, pepper, and herbs
- Top with a poached egg

3. Asian-Inspired Vegetable Stir Fry
- Sautéed celery, onions, potatoes
- Ginger-garlic base
- Sauce made with miso, soy sauce, and a touch of wasabi
- Garnished with cilantro
- Serve with scrambled eggs on the side

All these recipes avoid tomatoes and meat while incorporating eggs, which you enjoy. They make use of your available ingredients while respecting your vegetarian diet.



In the future, your array of kitchen robots that all have agents work together to cook one of these meals for you; pulling more personal info about you from project untitled if you give permission.



The End
