# **The Anime Recommendator v1.0**
![ANIME RECOMMENDATOR](/img/logo.png)
Hi everyone!
This is my personal anime recommendator program that constructs anime recommendation list, based on the anime database by KUMAR ARNAV on kaggle website!

Here is the link to this database:
https://www.kaggle.com/datasets/arnavvvvv/anime-dataset/data

---
### **Table of content**  

- [About Project](#about)
- [Files and functions](#filesandfunctions)
- [Reflection](#reflection)
- [Credits](#credits)
- [Final nots](#finalnotes)
---
## <a id="about">**This project was made by Aleksei Glebov (also known as Keysella) for DS 210 class at Boston University.**</a>

- **What was done in this project?**  
    - Basic logic to construct the anime recommendation list for a concrete user.
    - Program uses two databases and assumes that user with a provided name exists.
    - Database ***animes.csv*** was changed and filtered in a way that suits the project.
- **What can be improved in this project?**  
    - The recommendation logic can be improved in such a way:  
        - If we need to randomly select favorite genres.
        - If we need to make recommendation list taking into account other parameters (such as age or gender).
- **What can this program be used for?**  
    - Theoretically, this program can be used on a website with dinamical databases of the existing anime and the user's profile. Program can be connected to a website so that it can borrow the new information about constantly updating anime ratings - it will always be able to suggest new anime to watch.

---
### <a id="filesandfunctions">**The anime recommendator consists of 3 main coding parts that are located in *src* directory:**</a>
1. ***main.rs*** file - this is the main file of the program that will start the ***main()*** function which uses all of the other files **in order to construct the list of recommendations**.  
At the beginning of the function we need to choose the profile's name - this is the user's name for whom this program will make a recommendation list. Next, the program gets all the information about this user - it uses code from the ***work_with_profile.rs*** file, which reads the ***profiles.csv*** database and finds the user's string, and then builds the vector of strings with all the information about a concrete user. After program got the full information about the user - it checks if the user even has a **favorite anime list**. If user does have one, then the program uses functions from ***work_with_anime.rs*** file to construct the recommendation list BASED ON which anime the user liked already. However, if user does not have a favorite anime list, then the program uses same ***work_with_anime.rs*** file to build the list of top five most popular anime in the community.  
This file contains following functions:  
    - ***main()*** function.  
2. ***work_with_profiles.rs*** file - this is the second file from this project.  
My program assumes that the chosen profile name in the ***main.rs*** file exists. The functions from this file are used to construct the vector of strings which represents the information about a concrete user.  
This file contains following functions:  
    - ***find_profile()*** - this function gets the username entry and reads the ***profile.csv*** database to find the information about concrete user. The database ***profile.csv*** was not changed, and because of that I had to deal with a difficult file reading. This database uses comma as a separator, and that caused problems in reading. That is why there is a second function to deal with it.
    - ***fill_profile()*** - it parses the user's string into a vector of strings. I have discovered that there are few main states of a string that can cause problems, so this functions works through these cases and builds the final vector of length 5, which has:  
        1. profile's name;
        2. user's gender;
        3. user's birthday;
        4. list of favorite anime (which contains anime ids);
        5. link to user's profile.
    - ***make_anime_id_list()*** - it simply takes the string of user's favorite anime ids and creates a vector of integers, where each integer represents the id of a certain anime. If the list is empty, it returns the empty vector.
3. ***work_with_anime.rs*** file - this is the third file from this project.  
My program assumes that every anime from the database has all the information fields filled. Therefore, **program not check if a field is empty**.  
Code from this file is used to do the operations on the anime string from a data base. First goal is to find the anime on the list, and then work with it (in this version - mainly check for the genres of an anime).  
In addition to functions, this file includes a new structure named ***Genre*** that is used to store the name of a certain genre and the number of appearances of this genre in a user's favorite anime list.  
This file contains following functions:  
    - ***find_anime_by_id()*** - this function reads the ***animes.csv*** database to find the information about certain anime. Program assumes that every anime has its own id number, so it checks the first entry in the string to find the id.
    - ***get_anime_genres()*** - this function gets a vector of strings that represents the information about certain anime, and then returns a new vector of strings with the genres of this anime.
    - ***make_genres_list()*** - this function constructs the vector of string with all the genres that user faced while watching his favorite anime. This vector will contain duplicates.
    - ***genres_counter()*** - this function uses the structure ***Genre*** to build the vector of genres and their appearances in the users favorite anime list.
    - ***mk_topfive_from_nothing()*** - this function is used in a case when user does not have the favorite anime list. It will simply return the names of the most popular animes at the current time.
    - ***mk_topfive()*** - this is the most important function that contains the logic to construct the recommendation list. This logic can be improved or changed in a way that is asked, but for now it builds the recommendation list based on the genres that user's favorite anime contain.

---
### <a id="reflection">**Reflection**</a>
I enjoyed making this project come to live, and I really appreciate the opportunity that I was given.  
I like this program, and I may update it in the future.  
In my opinion, the logic that I built here is very simple, so I really want to improve it!

---
### <a id="credits">**Credits**</a>
While completing this project I used the following resources:
- DS 210 lecture notes;
- DS 210 Homeworks;
- My project proposal;
- rust-lang.org;
- stackoverflow.com;
- chat.openai.com;
- youtube.com.

---
### <a id="finalnotes">**Final notes**</a>
> There are two user's accounts on this project listed in "Contributors" section. I don't know how this happend, but my another account "Keysella" was added there. I think the problem was that I was pushing my files to github while working on my personal computer that has this account as the prior one.  
Aleksei Glebov and "Keysella" is one person, I didn't cheat while working on this project, and I did everything by myself. I have other accounts on social media named Keysella that can prove my identity.  

> I started working on this project on May 1, but I didn't commit anything on GitHub until May 5, 3 am when I saw on Gradescope that I must do that. The only evidence I have which concludes that I started this project not on the last day - is my Piazza post @461, where I asked if I can change my data files a little bit. I worked on this project for 5 days, and I have my full project proposal submitted on Gradescope that describes my project's details in a two-page document. Since I realized I need to show the commit history - I started the commit history from updating the ***README.md*** file, and if I have some time I will also improve my functions.  
Because of that, please show mercy while grading my commit history!
