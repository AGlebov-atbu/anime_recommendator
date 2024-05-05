use std::fs::File;
use std::io::{BufRead, BufReader};

// This is the structure Genre, that will be used to store the information
// for the apperences of a certain genre in user's anime list.
#[derive(Debug)]
#[derive(Clone)]
pub struct Genre {
    pub genre_name: String,
    pub counter: i32,
}

// Implement Genre structure and its methods.
impl Genre {
    fn new(genre_name: String) -> Genre {
        Genre {
            genre_name,
            counter: 1,
        }
    }
}

// This is my find_anime_by_id() function!
// This function reads the file of all animes on a website and finds
// the one by a provided name,
// then it returns a string related to that anime.
// This string contains all the information from this anime.
pub fn find_anime_by_id(file_name: &str, id: &i32) -> Vec<String> {
    // Read the file.
    let file = File::open(file_name).expect("Could not open file.");
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    // Prepare a temporary anime string.
    let mut anime = Vec::new();

    // Do the looping to find the anime.
    for line in lines {
        let line_str = line.unwrap();
        let fields: Vec<String> = line_str.trim().split(';')
                                          .map(|s| s.to_string())
                                          .collect();

        // First field in the file contains the anime id and can never be empty.
        let anime_id = &fields[0];
        // So once we found this name - we save the string that contains it.
        if anime_id.to_string() == id.to_string() {
            anime = fields;
        }
    }

    // Return the resulting vector that contains all the information about the anime.
    return anime;
}

// This is my function get_anime_genres() that returns the vector of genres of a certain anime!
pub fn get_anime_genres(anime: &Vec<String>) -> Vec<String> {
    // We know for sure the position of the genres, so we take the string of genres,
    // and then cut it to a new vector of strings with genres' names.
    let genres = anime[2].replace("[", "")
                                     .replace("]", "")
                                     .replace("'", "")
                                     .replace(" ", "")
                                     .split(',').map(|s| s.trim().parse().unwrap()).collect();
    return genres;
}

// This function makes a list that contains all the genres(including the duplicates).
pub fn make_genres_list(anime_list: &Vec<i32>, file_name: &str) -> Vec<String> {
    // Make the empty vector to fill it later.
    let mut genre_list: Vec<String> = Vec::new();

    // Work through every anime id.
    for anime_id in anime_list {
        let current_anime = find_anime_by_id(file_name, anime_id);
        let current_genres = get_anime_genres(&current_anime);
        genre_list.extend(current_genres);
    }

    return genre_list;
}

// This function counts the appearances of a certain genre and returns a vector of structure Genre!
pub fn genres_counter(genre_list: &Vec<String>) -> Vec<Genre> {
    // Prepare the genres vector to store the information about user's favorite genres.
    let mut genres: Vec<Genre> = Vec::new();

    // Work through every genre name.
    for genre_name in genre_list {
        // Check if the genre is already on the list.
        if let Some(genre) = genres.iter_mut().find(|g| &g.genre_name == genre_name) {
            // Increment counter if the genre is on the list.
            genre.counter += 1;
        } else {
            // If the genre is NOT on the list, just add it.
            genres.push(Genre::new(genre_name.to_string()));
        }
    }

    // Once we counted all the genres of a user, we return the vector.
    return genres;
}

// This is my mk_topfive_from_nothing() function!
// This function makes a top five of the popular anime.
// This function is a little bit of a different logic,
// because I think that a person who is new to anime should try watching most popular anime first.
// I fully understand that some anime are not for all, but the popularity parameter in the database means that
// MOST users liked this anime and added to their favorite list, so I think the logic is actually correct.
// The other topfive function will contain a different logic - it will make a top 5 based on anime rank,
// and not just the first five popular anime (also it will take into account and skip the anime that user already watched).
pub fn mk_topfive_from_nothing(file_name: &str) -> Vec<String> {
    // Prepare the top five vector to store the anime names.
    let mut topfive: Vec<String> = Vec::new();
    
    // Do the looping five times to complete the top.
    for i in 1..=5 {
        // Read the file.
        let file = File::open(&file_name).expect("Could not open file.");
        let buf_reader = BufReader::new(file);
        let lines = buf_reader.lines();

        // Do another looping that will simply return the anime with popularity i.
        for line in lines {
            let line_str = line.unwrap();
            let fields: Vec<String> = line_str.trim().split(';')
                                              .map(|s| s.to_string())
                                              .collect();

            // Find the popularity that is under the seventh column and compare it with i.
            if let Ok(anime_popularity) = fields[6].parse::<i32>() {
                if anime_popularity == i {
                    // Add the name of the anime to the vector.
                    topfive.push(fields[1].clone());
                    break;
                }
            }
        }
    }

    // Return the topfive vector with descending order.
    return topfive;
}

// This is my mk_topfive() function!
// This function makes a top five anime that a user should consider watching.
// The logic of this function right now is very simple - it tries to find highly rated anime,
// which a user does not have on the favorite anime list, and which contains the genres of user's favorite anime.
pub fn mk_topfive(favorite_anime_list: &Vec<i32>, favorite_genres: &Vec<Genre>, file_name: &str) -> Vec<String> {
    // Prepare the final vector for topfive anime.
    let mut topfive: Vec<String> = Vec::new();

    // We want to cut favorite genres so that we use only those which appeared the most times on the list.
    // Since the list of genres is already sorted in the descending order - we can simply take first five
    // and they will be considered as user's favorite genres.
    let mut fav_genres: Vec<Genre> = Vec::new();
    if favorite_genres.len() > 5 {
        for i in 0..5 {
            fav_genres.push(favorite_genres[i].clone());
        }
    }

    // Read the file.
    let file = File::open(file_name).expect("Could not open file.");
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    // Start searching for the anime.
    for line in lines {
        let line_str = line.unwrap();
        let fields: Vec<String> = line_str.trim().split(';').map(|s| s.trim().to_string()).collect();

        // Extract anime id and name from the line.
        let anime_id = match fields[0].parse::<i32>() {
            Ok(id) => id,
            // Skip the first line.
            Err(_) => continue,
        };
        // We know for sure that the anime name is in the second column, so:
        let anime_name = fields[1].clone();

        // Check if user has already watched this anime.
        // If the anime is in the favorite list - we skip it.
        if favorite_anime_list.contains(&anime_id) {
            continue;
        }

        // Check if anime contains one of the favorite genres.
        let mut found_genre = false;
        for genre in &fav_genres {
            if line_str.contains(&genre.genre_name) {
                found_genre = true;
                break;
            }
        }
        // Skip this anime if it doesn't contain any favorite genres.
        if !found_genre {
            continue;
        }

        // Add the anime to our topfive vector. The order is descending automatically,
        // because the database is filtered by rating in a descending order,
        // and we check animes from top to bottom.
        if topfive.len() < 5 {
            topfive.push(anime_name);
        }
    }

    // Return the final top.
    return topfive;
}
