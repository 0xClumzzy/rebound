pub const WALLPAPERS: &[(&str, &[u8])] = &[
    ("1.2.png", include_bytes!("wallpapers/1.2.png")),
    ("171e42199a3f46bbd10339bfcde4870a.jpg", include_bytes!("wallpapers/171e42199a3f46bbd10339bfcde4870a.jpg")),
    ("2.jpg", include_bytes!("wallpapers/2.jpg")),
    ("3_1.jpg", include_bytes!("wallpapers/3_1.jpg")),
    ("8TwRQp.png", include_bytes!("wallpapers/8TwRQp.png")),
    ("A_Tree_With_White_Flowers_In_The_Dark_Photo_Tokyo_Image_On.jpg", include_bytes!("wallpapers/A_Tree_With_White_Flowers_In_The_Dark_Photo_Tokyo_Image_On.jpg")),
    ("Aesthetic_Black_Lockscreen_Dark_Wallpaper_iPhone.jpg", include_bytes!("wallpapers/Aesthetic_Black_Lockscreen_Dark_Wallpaper_iPhone.jpg")),
    ("Aesthetic_Shounen_Anime_Black_And_White_iPhone_Wallpaper.jpg", include_bytes!("wallpapers/Aesthetic_Shounen_Anime_Black_And_White_iPhone_Wallpaper.jpg")),
    ("An_Eye_Catching_Black_And_White_Aesthetic_Wallpaper.jpg", include_bytes!("wallpapers/An_Eye_Catching_Black_And_White_Aesthetic_Wallpaper.jpg")),
    ("Anime_Collage_Black_And_White_Pfp_Wallpaper.jpg", include_bytes!("wallpapers/Anime_Collage_Black_And_White_Pfp_Wallpaper.jpg")),
    ("Atomic_Blast_Japanese_Aesthetic_Black_Wallpaper.jpg", include_bytes!("wallpapers/Atomic_Blast_Japanese_Aesthetic_Black_Wallpaper.jpg")),
    ("Awesome_Dark_Anime_HD_Wallpaper_1.jpg", include_bytes!("wallpapers/Awesome_Dark_Anime_HD_Wallpaper_1.jpg")),
    ("Black_Aesthetic_Wallpaper.jpeg", include_bytes!("wallpapers/Black_Aesthetic_Wallpaper.jpeg")),
    ("Black_Aesthetic_Wallpaper_For_Your_Phone_Prada.jpg", include_bytes!("wallpapers/Black_Aesthetic_Wallpaper_For_Your_Phone_Prada.jpg")),
    ("Black_And_White_Anime_Wallpaper_For_Desktop_Pc_By_Kawaiig0th3_On.jpg", include_bytes!("wallpapers/Black_And_White_Anime_Wallpaper_For_Desktop_Pc_By_Kawaiig0th3_On.jpg")),
    ("Black_Anime_Aesthetic_Pc_Wallpaper.jpg", include_bytes!("wallpapers/Black_Anime_Aesthetic_Pc_Wallpaper.jpg")),
    ("Black_Anime_Wallpaper_On.jpg", include_bytes!("wallpapers/Black_Anime_Wallpaper_On.jpg")),
    ("Black_Japanese_Wallpaper_Top_Background.jpg", include_bytes!("wallpapers/Black_Japanese_Wallpaper_Top_Background.jpg")),
    ("Black_Sad_Pictures_Image.jpg", include_bytes!("wallpapers/Black_Sad_Pictures_Image.jpg")),
    ("Boy_Dark_Aesthetic_Anime_Pfp_Wallpaper.jpg", include_bytes!("wallpapers/Boy_Dark_Aesthetic_Anime_Pfp_Wallpaper.jpg")),
    ("Broken_Heart_Aesthetic_Wallpaper_Black.jpg", include_bytes!("wallpapers/Broken_Heart_Aesthetic_Wallpaper_Black.jpg")),
    ("Dark_Aesthetic_Computer_With_Japanese_Characters.jpg", include_bytes!("wallpapers/Dark_Aesthetic_Computer_With_Japanese_Characters.jpg")),
    ("Dark_Collage_Anime_Phone_Wallpaper.jpg", include_bytes!("wallpapers/Dark_Collage_Anime_Phone_Wallpaper.jpg")),
    ("Dark_Japanese_iPhone_Wallpaper.jpg", include_bytes!("wallpapers/Dark_Japanese_iPhone_Wallpaper.jpg")),
    ("Dark_Wallpaper_For_iPhone_Background.jpg", include_bytes!("wallpapers/Dark_Wallpaper_For_iPhone_Background.jpg")),
    ("Depressed_Aesthetic_Phone_Wallpaper_On.jpg", include_bytes!("wallpapers/Depressed_Aesthetic_Phone_Wallpaper_On.jpg")),
    ("Deska_On_Upload_Gambar_Fotografi_Pemandangan.jpg", include_bytes!("wallpapers/Deska_On_Upload_Gambar_Fotografi_Pemandangan.jpg")),
    ("Photo.jpg", include_bytes!("wallpapers/Photo.jpg")),
    ("Photo_Wallpaper_Creative_Black_And_White_Girl_Water.jpg", include_bytes!("wallpapers/Photo_Wallpaper_Creative_Black_And_White_Girl_Water.jpg")),
    ("Sakura_Wallpaper_In_Cherry_Blossom_Japanese.jpg", include_bytes!("wallpapers/Sakura_Wallpaper_In_Cherry_Blossom_Japanese.jpg")),
    ("Style_Cewe_Black_Aesthetic_Wallpaper.jpg", include_bytes!("wallpapers/Style_Cewe_Black_Aesthetic_Wallpaper.jpg")),
    ("Top_Best_Black_Lock_Screen_For_iPhone.jpg", include_bytes!("wallpapers/Top_Best_Black_Lock_Screen_For_iPhone.jpg")),
    ("Wallpaper.jpg", include_bytes!("wallpapers/Wallpaper.jpg")),
    ("aesthetic-grunge-desktop-wallpaper-i1y44vn02gko2rei.jpg", include_bytes!("wallpapers/aesthetic-grunge-desktop-wallpaper-i1y44vn02gko2rei.jpg")),
];

pub fn deploy_all(home: &str) -> Vec<Result<String, String>> {
    let dir = format!("{}/Pictures/wallpapers", home);
    let _ = std::fs::create_dir_all(&dir);

    WALLPAPERS.iter().map(|(name, data)| {
        let path = format!("{}/{}", dir, name);
        match std::fs::write(&path, data) {
            Ok(()) => Ok(format!("deployed ~Pictures/wallpapers/{}", name)),
            Err(e) => Err(format!("failed ~Pictures/wallpapers/{}: {}", name, e)),
        }
    }).collect()
}
