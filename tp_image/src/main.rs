use image::{ImageFormat, Rgb, RgbImage, io::Reader as ImageReader};
use rand::Rng;

fn luminance(pixel: &Rgb<u8>) -> f32 {
    let [r, g, b] = pixel.0;
    0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32
}

fn threshold_to_monochrome(image: &RgbImage) -> RgbImage {
    let mut monochrome_image = image.clone();

    for pixel in monochrome_image.pixels_mut() {
        let lum = luminance(pixel);
        if lum > 128.0 {
            *pixel = Rgb([255, 255, 255]);
        } else {
            *pixel = Rgb([0, 0, 0]);
        }
    }

    monochrome_image
}

fn threshold_to_custom_colors(image: &RgbImage, color1: Rgb<u8>, color2: Rgb<u8>) -> RgbImage {
    let mut custom_image = image.clone();

    for pixel in custom_image.pixels_mut() {
        let lum = luminance(pixel);
        if lum > 128.0 {
            *pixel = color2; // Couleur pour "blanc"
        } else {
            *pixel = color1; // Couleur pour "noir"
        }
    }

    custom_image
}

fn color_distance(c1: &Rgb<u8>, c2: &Rgb<u8>) -> f32 {
    let [r1, g1, b1] = c1.0;
    let [r2, g2, b2] = c2.0;

    let r_diff = (r2 as f32 - r1 as f32).powi(2);
    let g_diff = (g2 as f32 - g1 as f32).powi(2);
    let b_diff = (b2 as f32 - b1 as f32).powi(2);

    (r_diff + g_diff + b_diff).sqrt()
}

fn apply_palette(image: &RgbImage, palette: Vec<Rgb<u8>>) -> RgbImage {
    if palette.is_empty() {
        eprintln!("Erreur : la palette est vide. Aucun changement appliqué à l'image.");
        return image.clone();
    }

    let mut palette_image = image.clone();

    for pixel in palette_image.pixels_mut() {
        let mut min_distance = f32::MAX;
        let mut closest_color = palette[0];

        for color in &palette {
            let distance = color_distance(pixel, color);
            if distance < min_distance {
                min_distance = distance;
                closest_color = *color;
            }
        }

        *pixel = closest_color;
    }

    palette_image
}

fn random_dithering(image: &RgbImage) -> RgbImage {
    let mut rng = rand::thread_rng(); // Initialisation du générateur de nombres aléatoires
    let mut dithered_image = image.clone();

    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            let lum = luminance(pixel); // Calcul de la luminosité du pixel
            let random_threshold: f32 = rng.gen_range(0.0..255.0); // Seuil aléatoire

            // Application du seuil pour déterminer la couleur du pixel
            dithered_image.put_pixel(
                x,
                y,
                if lum > random_threshold {
                    Rgb([255, 255, 255])
                } else {
                    Rgb([0, 0, 0])
                },
            );
        }
    }

    dithered_image
}

fn main() {
    // Partie 1
    let input_image_path = "images/1295367.jpg";
    let output_image_path = "images/output/output.png";

    let image = ImageReader::open(input_image_path)
        .expect("Impossible d'ouvrir le fichier")
        .decode()
        .expect("Erreur lors du décodage de l'image");

    // Convertir l'image en mode RGB8
    let rgb_image = image.to_rgb8();

    // Sauvegarder l'image en PNG
    match rgb_image.save_with_format(output_image_path, ImageFormat::Png) {
        Ok(_) => println!("Image sauvegardée avec succès dans {}", output_image_path),
        Err(err) => eprintln!("Erreur lors de la sauvegarde de l'image : {}", err),
    };

    // Lire la couleur d'un pixel spécifique (32, 52)
    let pixel = rgb_image.get_pixel(32, 52);
    println!("Couleur du pixel (32, 52) : {:?}", pixel);

    // Mettre un pixel sur deux en blanc
    let mut half_white_image = rgb_image.clone();
    for (x, y, pixel) in half_white_image.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 {
            *pixel = Rgb([255, 255, 255]);
        }
    }

    // Sauvegarder l'image modifiée
    let half_white_output_path = "images/output/half_white_output.png";
    match half_white_image.save_with_format(half_white_output_path, ImageFormat::Png) {
        Ok(_) => println!(
            "Image avec pixels blancs sauvegardée avec succès dans {}",
            half_white_output_path
        ),
        Err(err) => eprintln!(
            "Erreur lors de la sauvegarde de l'image avec pixels blancs : {}",
            err
        ),
    };

    // Partie 2
    let output_path_monochrome = "images/output/monochrome_output.png";
    let output_path_custom = "images/output/custom_output.png";  

    // Passer l'image en monochrome
    let monochrome_image = threshold_to_monochrome(&rgb_image);
    monochrome_image
        .save(output_path_monochrome)
        .expect("Failed to save monochrome image");

    // Remplacer "noir" et "blanc" par des couleurs personnalisées
    let color1 = Rgb([0, 128, 255]); // Bleu
    let color2 = Rgb([255, 255, 0]); // Jaune

    let custom_image = threshold_to_custom_colors(&rgb_image, color1, color2);
    custom_image
        .save(output_path_custom)
        .expect("Failed to save custom image");

    println!("Images sauvegardées avec succès dans {} et {}", output_path_monochrome, output_path_custom);

    // Partie 3
    let output_image_path = "images/output/output_palette.png";

    // Définir une palette de couleurs
    let palette = vec![
        Rgb([0, 0, 0]),      // Noir
        Rgb([255, 255, 255]), // Blanc
        Rgb([255, 0, 0]),     // Rouge
        Rgb([0, 255, 0]),     // Vert
        Rgb([0, 0, 255]),     // Bleu
        Rgb([255, 255, 0]),   // Jaune
        Rgb([255, 0, 255]),   // Magenta
        Rgb([0, 255, 255]),   // Cyan
    ];

    // Appliquer la palette à l'image
    let palette_image = apply_palette(&rgb_image, palette);

    // Sauvegarder l'image modifiée
    match palette_image.save_with_format(output_image_path, ImageFormat::Png) {
        Ok(_) => println!("Image avec palette sauvegardée avec succès dans {}", output_image_path),
        Err(err) => eprintln!("Erreur lors de la sauvegarde de l'image avec palette : {}", err),
    };

    // Partie 4 Dithering
    let output_dithered_path = "images/output/random_dithered_output.png";

    // Appliquer le dithering aléatoire
    let dithered_image = random_dithering(&rgb_image);

    // Sauvegarder l'image dithered
    match dithered_image.save_with_format(output_dithered_path, ImageFormat::Png) {
        Ok(_) => println!("Image avec dithering aléatoire sauvegardée avec succès dans {}", output_dithered_path),
        Err(err) => eprintln!("Erreur lors de la sauvegarde de l'image dithered : {}", err),
    };    
}
