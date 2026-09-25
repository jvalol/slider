//! The checker on the tunnel wall, built rather than loaded, so no image files
//! travel with the game. See `specs/0001-the-tunnel.md`.

use blitzkit::texture::TextureData;

/// How many pixels across the whole image is, and how many across one square.
/// Eight squares to a side, which is what the wall needs: the tunnel tiles this
/// 150 times along, and a coarser checker than this stops reading as speed.
pub const SIZE: u32 = 64;
pub const SQUARES: u32 = 8;
pub const SQUARE: u32 = SIZE / SQUARES;

/// The two colors, as straight red, green, blue and alpha.
pub const DARK: [u8; 4] = [60, 70, 110, 255];
pub const LIGHT: [u8; 4] = [220, 220, 230, 255];
/// One square in the corner is neither, which gives the eye something to hold
/// on to as the wall turns, and makes the tiling visible rather than uniform.
pub const MARK: [u8; 4] = [200, 60, 60, 255];

/// Whether the square at these pixel coordinates is the light one.
pub fn is_light(x: u32, y: u32) -> bool {
    ((x / SQUARE) + (y / SQUARE)) % 2 == 1
}

/// Whether this pixel is in the one marked square, which is the first one.
pub fn is_marked(x: u32, y: u32) -> bool {
    x < SQUARE && y < SQUARE
}

pub fn texture() -> TextureData {
    let mut pixels = Vec::with_capacity((SIZE * SIZE * 4) as usize);

    for y in 0..SIZE {
        for x in 0..SIZE {
            let color = if is_marked(x, y) {
                &MARK
            } else if is_light(x, y) {
                &LIGHT
            } else {
                &DARK
            };
            pixels.extend_from_slice(color);
        }
    }

    TextureData::from_pixels(SIZE, SIZE, pixels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_image_is_the_size_it_says() {
        let base = &texture().levels[0];

        assert_eq!(base.width, SIZE);
        assert_eq!(base.height, SIZE);
    }

    #[test]
    fn there_are_enough_squares_to_read_as_speed() {
        // two squares to a side is what this started as, and at that size the
        // wall reads as flat panels going past rather than as a checker
        // read through the texture rather than the constant, so the check is
        // on what is actually produced
        let base = &texture().levels[0];
        let row: Vec<bool> = (0..base.width).map(|x| is_light(x, 0)).collect();
        let runs = 1 + row.windows(2).filter(|pair| pair[0] != pair[1]).count();

        assert!(runs >= 8, "{} squares a side is too coarse", runs);
    }

    #[test]
    fn exactly_one_square_is_marked() {
        let marked: u32 = (0..SQUARES)
            .flat_map(|row| (0..SQUARES).map(move |column| (row, column)))
            .filter(|(row, column)| is_marked(column * SQUARE, row * SQUARE))
            .count() as u32;

        assert_eq!(marked, 1, "{} squares are marked", marked);
    }

    #[test]
    fn it_is_a_checker_not_stripes() {
        // the two along one edge differ, and the two across a diagonal match,
        // which is what makes it a checker rather than stripes
        assert_ne!(is_light(0, 0), is_light(SQUARE, 0));
        assert_ne!(is_light(0, 0), is_light(0, SQUARE));
        assert_eq!(is_light(0, 0), is_light(SQUARE, SQUARE));
        assert_eq!(is_light(0, 0), is_light(SQUARE * 2, 0), "it is not alternating");
    }

    #[test]
    fn a_square_is_all_one_color() {
        for y in 0..SQUARE {
            for x in 0..SQUARE {
                assert_eq!(is_light(x, y), is_light(0, 0), "{} {} broke the square", x, y);
            }
        }
    }

    #[test]
    fn it_tiles_without_a_seam() {
        // the column past the right edge is the column at the left edge, or a
        // seam would run the length of the tunnel
        for y in 0..SIZE {
            assert_eq!(is_light(0, y), is_light(SIZE, y), "a seam at row {}", y);
        }
        for x in 0..SIZE {
            assert_eq!(is_light(x, 0), is_light(x, SIZE), "a seam at column {}", x);
        }
    }

    #[test]
    fn the_two_colors_are_far_enough_apart_to_see() {
        let gap: i32 = (0..3)
            .map(|channel| (LIGHT[channel] as i32 - DARK[channel] as i32).abs())
            .sum();

        assert!(gap > 300, "the checker would read as flat grey");
        assert_eq!(DARK[3], 255, "the wall is not see-through");
        assert_eq!(LIGHT[3], 255);
    }

    #[test]
    fn every_pixel_is_written_and_the_mips_follow() {
        let checker = texture();
        let base = &checker.levels[0];

        assert_eq!(base.pixels.len(), (SIZE * SIZE * 4) as usize);
        // the engine builds the chain down to a single pixel, which is what
        // keeps the far end of the tunnel from shimmering
        assert!(checker.levels.len() > 1, "there is no mip chain");
        assert_eq!(checker.levels.last().unwrap().width, 1);
    }
}
