mod bike;
mod user;

use bike::Bike;
use user::User;


fn main() {
    let email = "sindani254@gmail.com";
    let mut user = User::new(email);
    println!("{} successfully logged in", user.email);

    let model = "Ninja 1000";
    let model2 = "Ninja 600";
    user.reg("Kawasaki", model, 0, "2022", 700000.00);

    user.reg("Kawasaki", model2, 10, "2020", 550000.00);

    println!("{:?}", user.show_bikes());
}


#[cfg(test)]
mod tests {
    use crate::user::User;
    use crate::bike::Bike;
    use super::*;

    #[test]
    fn create_user_object() {
        let email = "crimsoftit@gmail.com";
        let user = User::new(email);

        assert_eq!(&email, &user.email);
    }

    #[test]
    fn test_add_item_to_wishlist () {
        let email = "sindani254@gmail.com";
        let mut user = User::new(email);

        let model = "Ninja 1000";
        user.reg("Kawasaki", model, 0, "2022", 700000.00);

        assert_eq!(user.bikes.len(), 1);

        match user.bikes.get(&user.email) {
            Some(bike) => assert_eq!(model, &bike.model),
            None => println!("ERROR! test failed...")
        }
    }

    #[test]
    fn test_remove_item_from_wishlist () {
        let email = "sindani254@gmail.com";
        let mut user = User::new(email);
        user.remove_bike();

        assert_eq!(user.bikes.len(), 0);
    }
}
