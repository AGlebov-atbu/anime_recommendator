mod work_with_anime;
mod work_with_profile;

// This is my main() function that will perform all the operations
// to find a recommendation list for a concrete user!
// This program assumes that a user already has a list of favorite anime
// (since my database doesn't have a list of "watched" anime).
// In theory, this program can be used on the web site,
// where the more advanced version of it can be written.

// P.S.
//      There are some fields that are marked with comments like this:
// (!!!)
//      If you want - you can test their functionality by simply uncommenting them!
//      The main part of my anime_recommendator is uncommented already!
//      Please, enjoy! :)
fn main() {

    println!("\n\n\n------------------------------------------------------------------------------------------------------------------------------------------------------------");
    // File names for anime.csv and profile.csv.
    let file_anime = "databases/animes.csv";       // Do not change this file name.
    let file_profiles = "databases/profiles.csv"; // Do not change this file name.

    // Some profile name.
    // Next to this line there are some commented names that you can test the program with.
    // (All the tests were passed, but you can check it).
    let profile_name = "nyja-chan";
                           // Lucryllin
                           // eneri
                           // Pheefa
                           // shadowsplat
                           // SakuraSan628
                           // skankfish
                           // MrAnimeCrunch

    // Get all the information from the profile.
    // We use find_profile function, that will find the user's profile by its name, collect all the information about it,
    // and built the final vector of strings with that information.
    // This function builds the vector in such a way that there are exactly five entries,
    // so we always know which entry contains which information about user.
    // Unfortunately, some of the fields can be empty,
    // but what we know for sure is that there are always a user profile name and the url to his page.
    // Here are all the fields:
    //      0) User name;
    //      1) User's gender;                   (can be empty)
    //      2) User's date of birth;            (can be empty)
    //      3) User's favorite anime list;      (can be empty)
    //      4) Link to user's account.
    let profile = work_with_profile::find_profile(file_profiles, profile_name);

    // Here we print the information about user so that we know who we are working on.
    println!("Hi, {}!\nWe know that you are {}, and your date of birth is: {}.\nThe ids of your favorite anime are: {}.\nHere is the link to your account, if you forgot it: {}.",
            profile[0],profile[1],profile[2],profile[3],profile[4]);

    println!("------------------------------------------------------------------------------------------------------------------------------------------------------------");



    // Now we need to check if the user has favorite anime on the list.
    // If user doesn't have favorite anime, he will be given a regular top five best anime of all times
    // (which are simply first five best anime that were highly rated by others).
    if profile[3] != "" {

        // Find the favorite anime list, which is a vector of integers,
        // where each integer represents the id of a certain anime.
        let favorite_anime_list = work_with_profile::make_anime_id_list(profile);
        println!("Your favorite anime are:");
        for anime_id in &favorite_anime_list {
            println!("   -   {}", work_with_anime::find_anime_by_id(file_anime, anime_id)[1]);
        }

        println!("------------------------------------------------------------------------------------------------------------------------------------------------------------");

        // Construct a genre list, that will have all the genres with the appropriate counting for each genre in the anime list of a user.
        // In easier words, count the unique genres.
        let genre_list = work_with_anime::make_genres_list(&favorite_anime_list, file_anime);
        let mut counted_genres = work_with_anime::genres_counter(&genre_list);
        // Sort favorite genres by their appearance in the list in a descending order.
        counted_genres.sort_by(|a, b| b.counter.cmp(&a.counter));



        // (!!!)
        // See the genres of each anime
        // (uncomment this section to see them).
        // (!!!)
        // println!("The genres of your favorite anime are:");
        // let mut genres: Vec<String> = Vec::new();
        // for anime in &favorite_anime_list {
        //     let current_anime = work_with_anime::find_anime_by_id(file_anime, anime);
        //     genres = work_with_anime::get_anime_genres(&current_anime);
        //     println!("{:?}", genres);
        // }
        // println!("------------------------------------------------------------------------------------------------------------------------------------------------------------");

        // (!!!)
        // Here you can take a look at the list with counter
        // (uncomment this section if you want to see it).
        // (!!!)
        // println!("Genres and counters of your favorite anime:");
        // for genre in &counted_genres {
        //     println!("   -   Genre: {}   Count: {}", genre.genre_name, genre.counter);
        // }
        // println!("------------------------------------------------------------------------------------------------------------------------------------------------------------");

        // (!!!)
        // Print favorite genres
        // (uncomment this section if you want to see it).
        // (!!!)
        // println!("Your favorite genres based on the list are:");
        // for genre in &counted_genres {
        //     println!("   -   {}", genre.genre_name);
        // }
        // println!("------------------------------------------------------------------------------------------------------------------------------------------------------------");



        println!("Here is your top of five anime that you should try watching!\n");
        // Use mk_topfive() function to complete the topfive recommended anime.
        let topfive = work_with_anime::mk_topfive(&favorite_anime_list, &counted_genres, file_anime);
        // Print the top.
        for i in 0..5 {
            println!("   {})   {}", (i + 1), topfive[i]);
        }

    } else {
        // This is the case when a user doesn't have favorite anime,
        // so we simply recommend him the most popular anime.
        println!("Unfortunately, you do not have any favorite anime yet! :c\nWe will give you top five most popular anime that are currently out!");

        println!("\nConsider watching these anime:");
        let regular_topfive = work_with_anime::mk_topfive_from_nothing(file_anime);
        for i in 0..5 {
            println!("   {})   {}", (i + 1), regular_topfive[i]);
        }

        println!("\nWe hope you will enjoy it! :)\nHave a good luck on your adventure to the anime world!");
    }
    println!("------------------------------------------------------------------------------------------------------------------------------------------------------------");
    println!("\n\n\n");
}
