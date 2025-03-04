use tracing::info;

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Effect {
    damage: u32,
    continues: bool,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,
    Run(Direction),
    Teleport { x: u32, y: u32 },
    Attack {
        effect: Effect,
        position: (u32, u32),
    },
}

pub fn player_run() {
    info!("run in player mod");
    let actions = [
        PlayerMove::Pass,
        PlayerMove::Teleport { x: 100, y: 10 },
        PlayerMove::Run(Direction::LEFT),
        PlayerMove::Teleport { x: 1, y: 2 },
        PlayerMove::Run(Direction::RIGHT),
        PlayerMove::Attack {
            effect: Effect {
                damage: 10,
                continues: true,
            },
            position: (1, 2),
        },
        PlayerMove::Attack {
            effect: Effect {
                damage: 110,
                continues: true,
            },
            position: (1, 2),
        },
        PlayerMove::Attack {
            effect: Effect {
                damage: 80,
                continues: false,
            },
            position: (1, 2),
        }
    ];

    for item in actions.iter() {
        match item {
            PlayerMove::Pass => info!("FOR {}", "pass"),
            PlayerMove::Run(Direction::LEFT) => info!("you ran left"),
            PlayerMove::Run(direction) => info!("run {:?}", direction),
            PlayerMove::Teleport { x: xpos @ ..50, y } => {
                info!("teleport to small xpos {:?}, {:?}", xpos, y)
            },
            PlayerMove::Teleport { x, y } => info!("teleport to {:?}, {:?}", x, y),
            PlayerMove::Attack { effect: Effect { damage: dmg @ ..=30, .. }, .. } => {
                info!("attack damage {} < 30 too weak!", dmg);
            },
            PlayerMove::Attack { effect: Effect { damage, continues: true }, .. } => {
                info!("continues loss hp attack damage {:?}", damage)
            },
            PlayerMove::Attack { effect: effect @ Effect { damage, continues }, ..} => {
                info!("attack {:?} damage {:?} continues loss hp: {}", effect, damage, continues)
            },
        }
    }
}
