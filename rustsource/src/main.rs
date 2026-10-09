struct Ticket{
    number: u32,
    description: String,
    is_fixed: bool,
}
impl Ticket{
    fn new(number: u32, description: String) -> Ticket {
        Ticket {
            number,
            description,
            is_fixed: false,
        }
    }

    fn fix(&mut self) {
        self.is_fixed = true;
    }
 
    fn print(&self) {
        println!("#{} | {} | Fixed? {}",
            self.number,
            self.description,
            self.is_fixed
        );
    }
}

fn print_tickets(tickets: &[Ticket]) {
    for ticket in tickets {
        ticket.print();
    }
}

fn fix_ticket(tickets: &mut [Ticket], number: u32) {
    for ticket in tickets {
        if ticket.number == number{
            ticket.fix();
            return;
        }
    }

    println!("Can't find ticket!");
}

fn count_fixed(tickets: &[Ticket]) -> usize {
    let mut count = 0;

    for ticket in tickets {
        if ticket.is_fixed {
            count += 1;
        }
    }

    count
}

fn main() {
    let mut tickets: Vec<Ticket> = Vec::new();

    tickets.push(Ticket::new(
        1,
        String::from("wifi isnt working!!")
    ));
    tickets.push(Ticket::new(
        2,
        String::from("browser wont open")
    ));
    tickets.push(Ticket::new(
        3, 
        String::from("the printers doing it again")
    ));

    println!("Tickets before:");
    print_tickets(&tickets);

    fix_ticket(&mut tickets, 2);
    println!("~~~~~~~~~~~~~~");

    println!("Tickets after:");
    print_tickets(&tickets);

    println!("~~~~~~~~~~~~~~");
    println!("Total fixed: {}", count_fixed(&tickets));
}