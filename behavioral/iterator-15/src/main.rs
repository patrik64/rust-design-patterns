#[derive(Debug)]
pub struct MenuItem {
    name: String,
    description: String,
    vegetarian: bool,
    price: f64,
}

impl MenuItem {
    pub fn empty() -> Self {
        MenuItem::new("", "", false, 0.0)
    }

    pub fn new(name: &str, description: &str, vegetarian: bool, price: f64) -> Self {
        MenuItem {
            name: String::from(name),
            description: String::from(description),
            vegetarian, price
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn is_vegetarian(&self) -> bool {
        self.vegetarian
    }

    pub fn price(&self) -> f64 {
        self.price
    }
}

pub trait Menu {
    fn iter(&self) -> std::slice::Iter<'_, MenuItem>;
}

pub struct PancakeHouseMenu {
    menu_items: Vec<MenuItem>,
}

impl PancakeHouseMenu {
    pub fn new() -> Self {
       PancakeHouseMenu {
           menu_items: vec![
               MenuItem::new("K&B's Pancake Breakfast",
                             "Pancakes with scrambled eggs and toast",
                             true, 2.99),
               MenuItem::new("Regular Pancake Breakfast",
                             "Pancakes with fried eggs, sausage",
                             false, 2.99),
               MenuItem::new("Waffles",
                             "Waffles with your choice of blueberries or strawberries",
                             true, 3.59)
           ],
       }
    }

    pub fn add_item(&mut self, name: &str, description: &str,
                    vegetarian: bool, price: f64) {
        self.menu_items.push(MenuItem::new(name, description, vegetarian, price));
    }
}

impl Menu for PancakeHouseMenu {
    fn iter(&self) -> std::slice::Iter<'_, MenuItem> {
        self.menu_items.iter()
    }
}

const MAX_ITEMS: usize = 6;

pub struct DinerMenu {
    number: usize,
    menu_items: [MenuItem; MAX_ITEMS],
}

impl DinerMenu {
    pub fn new() -> Self {
        DinerMenu {
            number: 3,
            menu_items: [MenuItem::new("Vegetarian BLT",
                                       "(Fakin') Bacon with lettuce & tomato on whole wheat", true, 2.99),
                MenuItem::new("BLT",
                              "Bacon with lettuce & tomato on whole wheat", false, 2.99),
                MenuItem::new("Soup of the day",
                "Soup of the day, with a side of potato salad", false, 3.29),
                MenuItem::empty(), MenuItem::empty(), MenuItem::empty()],
        }
    }

    pub fn add_item(&mut self, name: &str, description: &str, vegetarian: bool, price: f64) {
        if self.number >= MAX_ITEMS {
            panic!("Sorry, menu is full!  Can't add item to menu");
        } else {
            self.menu_items[self.number] = MenuItem::new(name, description, vegetarian, price);
            self.number += 1;
        }
    }
}

impl Menu for DinerMenu {
    fn iter(&self) -> std::slice::Iter<'_, MenuItem> {
        self.menu_items[..self.number].iter()
    }
}

pub struct Waitress {
    menus: Vec<Box<dyn Menu>>,
}

impl Waitress {
    pub fn new(menus: Vec<Box<dyn Menu>>) -> Self {
       Waitress { menus }
    }

    pub fn print_menu(&self) {
       for menu in self.menus.iter() {
           for item in menu.iter() {
               println!("{:#?}", item);
           }
       }
    }
}

fn main() {
    let mut diner_menu = DinerMenu::new();
    diner_menu.add_item("Hotdog",
                        "A hot dog, with sauerkraut, relish, onions, topped with cheese",
                        false, 3.05);

    let mut pancake_menu = PancakeHouseMenu::new();
    pancake_menu.add_item("Blueberry Pancakes",
                          "Pancakes made with fresh blueberries and blueberry syrup",
                          true, 3.49);

    let waitress = Waitress::new(vec![
        Box::new(diner_menu),
        Box::new(pancake_menu)]);

    waitress.print_menu();
}