struct Parent {
    children: Vec<Child>,
    money: i32,
    cat: Cat,
}

struct Child{ 
    spend_total: i32,
}

struct Cat {
    spend_total: i32,
}

impl Parent {
    fn new(cat: Cat) -> Parent {
        Parent {
            children: Vec::new(),
            money: 100, // Initial value for money
            cat,
        }
    }
}

impl Child {
    fn new() -> Child {
        Child {
            spend_total: 0,
        }
    }

    fn spend(&mut self, parent_money: &mut i32, amount: i32) {
        if *parent_money >= amount {
            *parent_money -= amount;
            self.spend_total += amount;
            println!("Spent {} money, parent's remaining money: {}", amount, *parent_money);
        } else {
            println!("Not enough money!");
        }
    }
}

impl Cat {
    fn new() -> Cat {
        Cat {
            spend_total: 0,
        }
    }

    fn spend(&mut self, parent_money: &mut i32, amount: i32) {
        if *parent_money >= amount {
            *parent_money -= amount;
            self.spend_total += amount;
            println!("Spent {} money, parent's remaining money: {}", amount, *parent_money);
        } else {
            println!("Not enough money!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cat = Cat::new();
        let mut parent = Parent::new(cat);

        let child1 = Child::new();
        let child2 = Child::new();

        parent.children.push(child1);
        parent.children.push(child2);

        let index = 0;
        // let c1 = &mut parent.children[index];     // 对 parent.children 的可变借用
        let c1 = get_child_v2(&mut parent.children, index);
        c1.spend(&mut parent.money, 10); // 对 parent.money 的可变借用
        c1.spend(&mut parent.money, 20);
    }

    fn get_child<'a>(parent: &'a mut Parent, index: usize) -> &'a mut Child {
        &mut parent.children[index]
    }

    fn get_child_v2(children: &mut [Child], index: usize) -> &mut Child {
        &mut children[index]
    }
}