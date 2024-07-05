/**
 * @Author -> AryanRanjanae(ranjanearyan14@gmail.com)
 * @Brief -> Demonstration of Enums
 * @Date -> 6/4/24
 */

enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    /* main function  */
    let my_direction = Direction::North;
    move_around(my_direction);
}

fn move_around(direction: Direction) {
    // implements a logic to move north
}
