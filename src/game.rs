pub trait Game {
    type Coordinate;

    type InputCoordinate;

    type Player;

    type Score: PartialEq + PartialOrd;

    fn get(&self, coordinate: Self::Coordinate) -> Option<&Self::Player>;

    fn play(&mut self, player: Self::Player, coordinate: Self::InputCoordinate) -> Result<(), &str>;

    fn get_score(&self, player: Self::Player) -> Self::Score;

    fn is_full(&self) -> bool;

    fn print(&self);
}