# **The Anime Recommendator v1.0**
![](/img/logo.png)
Hi everyone!
This is my personal anime recommendator program that constructs anime recommendation list, based on the anime database by KUMAR ARNAV on kaggle website!

Here is the link to this database:
https://www.kaggle.com/datasets/arnavvvvv/anime-dataset/data

## **This project was made by Aleksei Glebov (also known as Keysella) for DS 210 class at Boston University.**

- **What was done?**  
    - Basic logic to construct the anime recommendation list for a concrete user.
    - Program uses two databases and assumes that user with a provided name exists.
- **What can be improved in this project?**  
    - The recommendation logic can be improved in such a way:  
        - If we need to randomly select favorite genres.
        - If we need to make recommendation list taking intop account other parameters (such as age or gender).
- **What can this program be used for?**  
    - Theoretically, this program can be used on a website which will provide dinamical databases of the existing anime and the user's profile. If we work on that program, we can connect it so that it can borrow the new information and constantly updating anime ratings - so that it can always suggest new anime to watch.

### **The anime recommendator consists of 3 main coding parts that are located in src directory:**
1. ***main.rs*** file - this is the main file of the program that will start the ***main()*** function which uses all of the other files **in order to construct the list of recommendations**.  
At the beginning of the function we need to choose the profile's name - this is the user's name for whom this program will make a recommendation list. Next, the program gets all the information about this user - it uses code from the ***work_with_profile.rs*** file, which reads the ***profiles.csv*** database and finds the user's string, and then builds the vector of strings with all the information about a concrete user. After program got the full information about the user - it checks if the user even has a **favorite anime list**. If user does have one, then the program uses functions from ***work_with_anime.rs*** file to construct the recommendation list BASED ON which anime the user liked already. However, if user does not have a favorite anime list, then the program uses same ***work_with_anime.rs*** file to build the list of top five most popular anime in the community.  
The ***main.rs*** file has the following functions:  
    - ***main()*** function.  
2. ***work_with_profiles.rs*** file - this is the second file from this project.  
My program assumes that the chosen profile name in the ***main.rs*** file exists. The functions from this file are used to construct the vector of strings which represents the information about a concrete user.  
The ***work_with_profiles.rs*** file has the following functions:  
    - ***find_profile()*** - this function gets the username entry and reads the ***profile.csv*** database to find the information about concrete user. The database ***profile.csv*** was not changed, and because of that I had to deal with a difficult file reading. This database uses comma as a separator, and that caused problems in reading. That is why there is a second function to deal with it.
    - ***fill_profile()*** - it parses the user's string into a vector of strings. I have discovered that there are few main states of a string that can cause problems, so this functions works through these cases and builds the final vector of length 5, which has:  
        1. profile's name;
        2. user's gender;
        3. user's birthday;
        4. list of favorite anime (which contains anime ids);
        5. link to user's profile.
    - ***make_anime_id_list()*** - it simply takes the string of user's favorite anime ids and creates a vector of integers, where each integer represents the id of a certain anime. If the list is empty, it returns the empty vector.

I started working on this project on May 1, but I didn't commit anything on GitHub (because I forgot to do that). The only evidence I have which concludes that I started this project not on the last day - is my Piazza post @461, where I asked if I can change my data files a little bit