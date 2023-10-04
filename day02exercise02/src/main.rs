#[derive(Debug)]
struct Unicorn{
name:String,
magic_power:u32,
}

#[derive(Debug)]
struct Griffin{
name:String,
magic_power:u32,
}

#[derive(Debug)]
enum Creature{
    Unicorn(Unicorn),
    Griffin(Griffin),
}



fn compare_magic<'a>(creature1:&'a Creature,creature2:&'a Creature)->&'a Creature{
    let power1=creature1.get_power();
    let power2=creature2.get_power();

    if power1 > power2{
        return creature1;
    }else if power2 > power1{
        return creature2;
    }else{
        return creature1;
    }  
}

impl Creature{
    fn get_name(&self)->&str{
        match self{
            Creature::Unicorn(u)=>&u.name,
            Creature::Griffin(g)=>&g.name,
        }
    }

    fn get_power(&self)->u32{
        match self{
            Creature::Unicorn(u)=>u.magic_power,
            Creature::Griffin(g)=>g.magic_power,
        }
    }
    fn clone(&self)->Self{
        match self{
            Creature::Unicorn(u)=>Creature::Unicorn(Unicorn{name:self.get_name().to_owned(),magic_power:self.get_power()}),
            Creature::Griffin(u)=>Creature::Griffin(Griffin{name:self.get_name().to_owned(),magic_power:self.get_power()}),
        }
    }

    fn get_boxed(&self)->Box<Self>{
        Box::new(self.clone())
    }
}


fn main() {
    let creature1=Creature::Unicorn(Unicorn{name:"Unicorn A".to_owned(),magic_power:3});
    let creature2=Creature::Unicorn(Unicorn{name:"Unicorn B".to_owned(),magic_power:4});
    let creature3=Creature::Griffin(Griffin{name:"Griffin A".to_owned(),magic_power:3});
    let higher_creature=compare_magic(&creature1,&creature2);
    println!("Creature with higher power is {}",higher_creature.get_name());
    let higher_creature=compare_magic(&creature1,&creature3);
    println!("Creature with higher power is {}",higher_creature.get_name());

    let boxed=creature1.get_boxed();
    println!("Boxed is {:?}",boxed);
    
  //  println!("Creature {} has more power with {}",&creature1.name,&creature1.magic_power);
}
