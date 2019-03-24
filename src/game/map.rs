use rand::Rng;

pub type Map = Vec<Vec<Tile>>;

#[derive(Copy, Clone)]
pub struct Tile {
    pub is_wall: bool,
    pub is_see: bool,
}

struct Room {
    loc: (u32, u32),
    size: (u32, u32),
}

pub fn create_map(size: (u32, u32), max_rooms: u32, room_size: (u32, u32)) -> (Map, (f64, f64)) {
    let mut map = Map::new();
    for i in 0..size.0 {
        map.push(Vec::new());
        for j in 0..size.1 {
            map[i as usize].push(Tile {
                is_see: false,
                is_wall: true,
            })
        }
    }
    let mut rooms: Vec<Room> = Vec::new();
    let mut rng = rand::thread_rng();
    let mut starting = (0.0, 0.0);
    for room_number in 0..max_rooms {
        let mut room = Room {
            loc: (rng.gen_range(0, size.0), rng.gen_range(0, size.1)),
            size: (
                rng.gen_range(room_size.0, room_size.1),
                rng.gen_range(room_size.0, room_size.1),
            ),
        };
        if rooms.is_empty() {
            starting = (
                room.loc.0 as f64 + room.size.0 as f64 / 2.0,
                room.loc.1 as f64 + room.size.1 as f64 / 2.0,
            );
        }
        if !any_overlap(&rooms, &room) {
            rooms.push(room);
        }
    }

    for rl in 0..rooms.len() {
        let r = &rooms[rl];
        for i in 0..r.size.0 {
            for j in 0..r.size.1 {
                let x = r.loc.0 + i;
                let y = r.loc.1 + j;
                if x > 0 && x < size.0 && y > 0 && y < size.1 {
                    map[x as usize][y as usize] = Tile {
                        is_wall: false,
                        is_see: true,
                    };
                }
            }
        }
        if rl > 0 {
            let rt = &rooms[rl - 1];
            let mid_r = (r.loc.0 + r.size.0 / 2, r.loc.1 + r.size.1 / 2);
            let mid_rt = (rt.loc.0 + rt.size.0 / 2, rt.loc.1 + rt.size.1 / 2);
            if rng.gen() {
                let x = mid_r.0;
                for y in mid_r.1..mid_rt.1 {
                    if x > 0 && x < size.0 && y > 0 && y < size.1 {
                        map[x as usize][y as usize] = Tile {
                            is_wall: false,
                            is_see: true,
                        };
                    }
                }
                let y = mid_rt.1;
                for x in mid_r.0..mid_rt.0 {
                    if x > 0 && x < size.0 && y > 0 && y < size.1 {
                        map[x as usize][y as usize] = Tile {
                            is_wall: false,
                            is_see: true,
                        };
                    }
                }
            } else {
                let y = mid_r.1;
                for x in mid_r.0..mid_rt.0 {
                    if x > 0 && x < size.0 && y > 0 && y < size.1 {
                        map[x as usize][y as usize] = Tile {
                            is_wall: false,
                            is_see: true,
                        };
                    }
                }
                let x = mid_rt.0;
                for y in mid_r.1..mid_rt.1 {
                    if x > 0 && x < size.0 && y > 0 && y < size.1 {
                        map[x as usize][y as usize] = Tile {
                            is_wall: false,
                            is_see: true,
                        };
                    }
                }
            }
        }
    }

    (map, starting)
}

fn any_overlap(rooms: &Vec<Room>, room: &Room) -> bool {
    for room_1 in rooms.iter() {
        if room.loc.0 < room_1.loc.0 + room_1.size.0
            && room.loc.0 + room.size.0 > room_1.loc.0
            && room.loc.1 < room_1.loc.1 + room_1.size.1
            && room.loc.1 + room.size.1 > room_1.loc.1
        {
            return true;
        }
    }
    false
}
