mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Direction {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Keypad{
    keypad_i: [[u8; 3]; 3],
    keypad_ii: [[u8; 5]; 5],
    directions: [Direction; 4],
    passcode:Vec<String>,
}

#[wasm_bindgen]
impl Keypad{

    pub fn render(&self) -> String {
        self.to_string()
    }

    fn get_bathroom_codes(&self) -> String {
        let codes= "UULDRRRDDLRLURUUURUURDRUURRDRRURUDRURRDLLDRRRDLRUDULLRDURLULRUUURLDDRURUDRULRDDDUDRDLDDRDDRUURURRDDRLRLUDLUURURLULLLRRDRLDRLRDLULULRDRDDUURUDRRURDLRRDDDLUULDURDLDLLRLRLLUDUDLRDDLUURUUDDRDULDDLDLLDULULRLDDDUDDDRLLRURLRDUUUDUUDDURRDLDDLRDLLUDDLDRLDULDRURLUUDLURLUDRULRLRUUUURLUUUDDULLRLLURDRURLLRLRLDDRURURULRULLUUUULUDULDDDRDDLURLUURRLDDRDRUDDRRLURRDURRLDUULRRLLRDLLDDUURULLRUURRRRDRRURLULLRLRDDULULRDLDDLULLD
        UUDUDDRRURRUDDRLDLURURLRLLDRLULLUURLLURDRLLURLLRRLURDLDURUDRURURDLRDRRDULRLLLRDLULDRLLDLDRLDDRUUUUULRLDUURDUUUURUUDLRDLLDRLURULDURURLDLLRDLULLULLLLLUDUDDLRLLLUDLRUUDDUUDUDDDLULDDUDUULUUDUDRRULRRRURUDUUULDDRURLLULLULURLUDRDLUUUDLDRRLRRRULLRRURRUDDDRDLDDDLDUDLLDRRDURRURRURRLDLURUULRLDLUDUDUUULULUUDDDLDDULRDULLULDRDDURRURRRULRDURULUDURRDLLUURRUURLLLULDRRULUUUURLRLRDDDDULLUUUDRRLRRLRRLLLUDDDLRDDURURRDULLLUDLUDURRLRDURUURURDRDUUURURRUDRURRULLDDURRLRRRUULDRLDRRURUDLULRLLRRDLDDRLRRULDDLLUURUDDUDRLUD
        DDDUDDRRDRRRUULDRULDLDLURRRUURULRUDDRLLLLURRLRULDLURRULDRUDRRLLLLDULRDLUUURDDLDLURRLLUUURLLUDLUDRRDDULLULURDULRRDLRLDRRUUUUDLRRDLDDLDULDRUULRLLDLRURRUDLDDDRUUULLDDLULDULDUURUDDDLULUDLUURLRURUURDDUDRRLDRRRDDDDRDLUDRRDURDLDRURDDDRRLLLRDDRRRDDLDRLLUURRLDRDDRDLRDDLLDRLRDRDDDURLULLRUURDLULRURRUUDLDRLDRRDDRLDDUULLRDDRRLLLDDDUURDUDRUDUDULDULRUURLDURRDLUURRDLLDDLLURUUUDRLUURRDLUDUULRURLUDDLLRUDURRDRRRDRDLULRRLRUDULUUDRLURRRRLULURRDLLDRDDRLULURDURRDUUULLRDUUDLDUDURUDRUDDLRLULRLRLRRRLRUULLDDLUDDLDRRRLDDLLRLRLRUDULRLLLUULLDRDLDRRDULLRRLLDLDUDULUDDUUDLRDRLUUULLRLDLDDLLRUDDRDD
        DDUURRLULDLULULLDUDDRURDDRLRDULUURURRLURDLRRDUUDLULDRDLDLRLULLRULLDRLDRRULUDRLDURUURLLDLLDDLUULLRLRULRLUURDDDDDRLDRLLLDLULDLDLULRRURLLLLLLRLUDLRRLRULUULLLLURDLLRLLDDUDLLULDLLURUUDLRDRDUDDDRDDUULRLLDDDLLRLURLUDLULRRUUUULLDLDLLLDRLUDRDRDLUDLRUDRDRUDRDLLDDLRRLRDLDURDLDRUUUDRLULUULDURDLUUUDDDDDLDRDURDLULDDLLUDUURRUDDLURUDDLRLUUDURUDUULULUDLDLUURDULURURULDDDLUUUUDLUUDUDLLLRDDLRDDLRURRRLLLULLURULLRDLLDRULRDDULULRLUDRRRDULRLLUDUULLRDRDDDULULRURULDLDLDRDLDUDRDULLUUUUUDLRDURDUUULLLRUULLRUULDRRUUDLLLULLUURLDDLUULLRLRLRDRLLLRLURDDURUDUULULDLRLRLLUDURRURDRUDLRDLLRDDRDUULRDRLLRULLUDDRLDLDDDDUDRDD
        URDLUDUDLULURUDRLUDLUDLRLRLLDDDDDLURURUURLRDUDLRRUUDUURDURUULDRRRDDDLDUURRRDLRULRRDLRUDUDLDDDLLLRLRLRUUUUUULURRRLRLUDULURLDLLDUUDDRUDLDUDRRLULLULLDURDDRRLLRLDLLLLRLULLDDDDLDULLRDUURDUDURRUULLDRULUDLUULUUDDLDDRDLULLULDLDRLDLRULLRLURDURUDRLDURDRULRLLLLURRURLRURUDUDRRUDUUDURDDRRDRLURLURRLDRRLLRLRUDLRLLRLDLDDRDLURLLDURUDDUUDRRLRUDLUDULDRUDDRDRDRURDLRLLRULDDURLUUUUDLUDRRURDDUUURRLRRDDLULLLDLRULRRRLDRRURRURRUUDDDLDRRURLRRRRDLDLDUDURRDDLLLUULDDLRLURLRRURDRUULDDDUDRDRUDRRLRLLLLLURDULDUDRLULDRLUULUDDDDUDDRDDLDDRLLRULRRURDDDRDDLDLULRDDRRURRUDRDDDDRURDRRURUUDUDDUURULLDRDULURUDUD";
        String::from(codes)
    }

    pub fn new() -> Keypad{
        let keypad_i=[[1, 2, 3], [4, 5, 6], [7,8,9]];
        let keypad_ii=[[0, 0, 1, 0, 0], [0, 2, 3, 4, 0], [5,6,7,8,9],[0,10,11,12,0],[0,0,13,0,0]];
        let directions=[
            Direction { x: 0, y: -1 }, //U
            Direction { x: 1, y: 0 },  //R
            Direction { x: 0, y: 1 },  //D
            Direction { x: -1, y: 0 }, //L
        ];
        let passcode:Vec<String>=Vec::new();

        Keypad{
            keypad_i,
            keypad_ii,
            directions,
            passcode
        }
    }

    pub fn enter_keycode(&mut self,keypad_id:u8){
        let (mut x, mut y) = match keypad_id{
            1 => (1,1),
            2 => (0,2),
            _ => (0,0)
        };
        let codes = self.get_bathroom_codes();

        for line in codes.lines() {
            let codeset: Vec<&str> = line
            .split("")
            .filter(|x| !x.is_empty())
            .collect();
            
            for code in codeset.iter() {
                let (mut dx, mut dy) = (0, 0);
                match code {
                    &"U" => {
                        dx = x + self.directions[0].x;
                        dy = y + self.directions[0].y;
                    }
                    &"R" => {
                        dx = x + self.directions[1].x;
                        dy = y + self.directions[1].y;
                    }
                    &"D" => {
                        dx = x + self.directions[2].x;
                        dy = y + self.directions[2].y;
                    }
                    &"L" => {
                        dx = x + self.directions[3].x;
                        dy = y + self.directions[3].y;
                    }
                    _ => (),
                }
    
    
                if (dx >= 0 && dx < 3) && (dy >= 0 && dy < 3) && keypad_id== 1 {
                    x = dx;
                    y = dy;
                }else if (dx >= 0 && dx < 5) && (dy >= 0 && dy < 5) && keypad_id == 2{
                    if self.keypad_ii[dy as usize][dx as usize] != 0{
                        x = dx;
                        y = dy;
                    }else{
                        continue;
                    }
                }
            }
            if keypad_id == 1{
                print!("{}", self.keypad_i[y as usize][x as usize]);
                self.passcode.push(self.keypad_i[y as usize][x as usize].to_string());
            }else if keypad_id == 2{
                match self.keypad_ii[y as usize][x as usize]{
                    10 => self.passcode.push(String::from("A")),
                    11 => self.passcode.push(String::from("B")),
                    12 => self.passcode.push(String::from("C")),
                    13 => self.passcode.push(String::from("D")),
                    _=> self.passcode.push(self.keypad_ii[y as usize][x as usize].to_string())
                }
            }
        }
    }

}

impl fmt::Display for Keypad{
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
        for key in self.passcode.as_slice().chunks(8){
            for c in key{
                write!(f, "{}",c)?;
            }
        }
        write!(f, "\n")?;
        Ok(())
    }
}



