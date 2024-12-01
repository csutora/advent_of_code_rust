fn get_cache_dir() -> std::path::PathBuf {
    directories::ProjectDirs::from("com", "csutora", "advent_of_code_inputs")
        .map(|proj_dirs| proj_dirs.cache_dir().to_path_buf())
        .or_else(|| {
            directories::UserDirs::new()
                .map(|dirs| dirs.home_dir().join(".cache/advent_of_code_inputs"))
        })
        .unwrap_or_default() // returns nonexistent path if nothing works
}

fn get_cache_path(year: u64, day: u8) -> std::path::PathBuf {
    get_cache_dir().join(format!("{}-{:02}.txt", year, day))
}

fn read_session_token() -> String {
    dotenvy::dotenv().ok();
    std::env::var("ADVENT_OF_CODE_SESSION_TOKEN") // add this to .env!
        .expect("ADVENT_OF_CODE_SESSION_TOKEN not set in .env file") // yes we do want to panic here
}

pub fn get_for(year: u64, day: u8) -> Vec<String> {
    // if cache exists, return the cached data
    let cache_path = get_cache_path(year, day);
    if cache_path.exists() {
        return std::fs::read_to_string(cache_path)
            .expect("failed to read cache file even though it seems to exist")
            .lines()
            .map(String::from)
            .collect();
    }

    // otherwise request the data from the advent of code server
    let body = ureq::get(&format!("https://adventofcode.com/{year}/day/{day}/input"))
        .set("Cookie", &format!("session={}", read_session_token()))
        .call()
        .expect("failed to fetch input")
        .into_string()
        .expect("failed to read response");

    let lines: Vec<String> = body.lines().map(String::from).collect();
    assert!(!lines.is_empty(), "received empty input");

    // and save it to cache so next time it doesn't have to get requested
    std::fs::create_dir_all(cache_path.parent().unwrap())
        .and_then(|_| std::fs::write(&cache_path, &body))
        .expect("failed to cache input");

    lines
}
