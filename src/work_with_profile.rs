use std::fs::File;
use std::io::{BufRead, BufReader};

// Here is my fill_profile() function!
// This function converts the profile string to a vector of strings, which will be used
// to extract information easier.
fn fill_profile(temp_profile: Vec<String>) -> Vec<String> {
    // Prepare profile's fields.
    //      profile[0] = name
    //      profile[1] = gender
    //      profile[2] = birthday
    //      profile[3] = animelist
    //      profile[4] = url
    let mut profile: Vec<String> = vec!["".to_string(); 5];
    let mut date: String = ("").to_string();
    let mut animelist: String = ("").to_string();
    let url: String;
    // We also need iterator to iterate over the vector.
    let mut i: usize;

    // Ho-ho, let's go.
    // I discovered a certain amount of possible states of a string.
    // Each profile always contains a name and a url, which are first and last items.
    // However, we need to find gender, date of birth, and animelist,
    // and because these fields can potentially be empty(or date can be written differently)
    // we need to handle some cases.
    if temp_profile[2] == "" {  // Case when a user doesn't have a date of birth.
        if temp_profile[3] != "" {  // If this user has a certain number of favorite anime - we save it in a list.
            i = 3;
            
            while !temp_profile[i].contains("myanimelist.net") {
                animelist += &temp_profile[i];
                animelist += ",";
                i += 1;
            }
            animelist = animelist.replace("\"", "")
                                 .replace("[", "")
                                 .replace("'", "")
                                 .replace("],", "");
            // We update url after we finished with the list.
            url = format!("{}", temp_profile[i]);
        } else {    //If there is no favorite anime - we simply update the url.
            url = format!("{}", temp_profile[4]);
        }
    // Another case occurs when there is a date of birth.
    // I discovered that there are two possible options to be a date:
    } else {
        // 1) date doesn't start with the " symbol.
        // This means that there is actually only month and day, so we add it.
        // After the date has been added, we continue with the same logic
        // to find the anime list.
        if !temp_profile[2].starts_with("\"") {
            date = format!("{}", temp_profile[2]);

            if temp_profile[3] != "" {
                i = 3;
                
                while !temp_profile[i].contains("myanimelist.net") {
                    animelist += &temp_profile[i];
                    animelist += ",";
                    i += 1;
                }
                animelist = animelist.replace("\"", "")
                                     .replace("[", "")
                                     .replace("'", "")
                                     .replace("],", "");
                url = format!("{}", temp_profile[i]);
            } else {
                url = format!("{}", temp_profile[4]);
            }
        // 2) Second case happens when date 
        } else {
            date = format!("{},{}", temp_profile[2], temp_profile[3]).replace("\"", "");

            if temp_profile[4] != "" {
                i = 4;
                
                while !temp_profile[i].contains("myanimelist.net") {
                    animelist += &temp_profile[i];
                    animelist += ",";
                    i += 1;
                }
                animelist = animelist.replace("\"", "")
                                     .replace("[", "")
                                     .replace("'", "")
                                     .replace("],", "");
                url = format!("{}", temp_profile[i]);
            } else {
                url = format!("{}", temp_profile[5]);
            }
        }
    }
    // Assign profile data to the correct positions.
    profile[0] = format!("{}", temp_profile[0]);
    profile[1] = format!("{}", temp_profile[1]);
    profile[2] = date;
    profile[3] = animelist;
    profile[4] = url;

    return profile;
}

// This is my find_profile() function!
// This function reads the file of profiles on a website and finds
// the profile by a provided profile name,
// then it returns a string related to that profile.
// This string contains all the information from that profile,
// and will be cut in the other function to a vector of strings.
pub fn find_profile(file_name: &str, profile_name: &str) -> Vec<String> {
    // Read the file.
    let file = File::open(file_name).expect("Could not open file.");
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    // Prepare a temporary profile vector.
    let mut temp_profile = Vec::new();

    // Do the looping to find the profile.
    for line in lines {
        let line_str = line.unwrap();
        let fields: Vec<String> = line_str.trim().split(',')
                                          .map(|s| s.to_string())
                                          .collect();

        // First field in the file contains a profile name and can never be empty.
        let user = &fields[0];
        // So once we found this name - we save the string that contains it.
        if user == profile_name {
            temp_profile = fields;
        }
    }
    
    // Use fill_profile() function with a temporary string to finish profile parsing.
    let profile = fill_profile(temp_profile);

    // Return the resulting vector that contains all the information about current profile.
    return profile;
}

// Here is my make_anime_id_list() function!
// This is a simple function that just returns a vector of integers, where
// each integer represents the id of a certain anime.
// This is a small helper function, so that it is easier to find a concrete anime.
pub fn make_anime_id_list(profile: Vec<String>) -> Vec<i32> {
    // It takes a profile vector, takes the fourth entry of it which represents a string with anime ids,
    // then it makes a vector of integers from that string.
    let result: Vec<i32> = profile[3].replace(" ", "").split(',')
                                                    .map(|s| s.trim().parse().unwrap())
                                                    .collect();

    return result;
}
