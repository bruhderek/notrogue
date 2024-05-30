use notcurses::Plane;

pub fn get_center_xy(cli: &Plane) -> (u32, u32) {
    ((cli.size().w() as u32-1)/2, (cli.size().h() as u32-1)/2)
}