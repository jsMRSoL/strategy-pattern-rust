mod random;

use random::Random;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn generate_id(length: u8) -> String {
    Random::rand_alpha(length)
}

struct SupportTicket {
    id: String,
    customer: String,
    issue: String,
}

impl SupportTicket {
    fn new(customer: &str, issue: &str) -> Self {
	Self {
	    id: generate_id(8),
	    customer: customer.to_string(),
	    issue: issue.to_string(),
	}
    }
}

struct CustomerSupport {
    tickets: Vec<SupportTicket>,
    processing_strategy: String,
}

impl CustomerSupport {
    fn new(processing_strategy: &str) -> Self {
	Self {
	    tickets: Vec::new(),
	    processing_strategy: processing_strategy.to_string(),
	}
    }

    fn create_ticket(&mut self, customer: &str, issue: &str) {
	self.tickets.push(SupportTicket::new(customer, issue));
    }

    fn process_tickets(&mut self) {
	if self.tickets.len() == 0 {
	    println!("There are no tickets to process. Well done!");
	    return;
	}

	if self.processing_strategy == "fifo" {
	    for ticket in &self.tickets {
		self.process_ticket(ticket);
	    }
	} else if &self.processing_strategy == "filo" {
	    self.tickets.reverse();
	    for ticket in &self.tickets {
		self.process_ticket(ticket);
	    }
	} else {
	    self.tickets.shuffle(&mut thread_rng());
	    for ticket in &self.tickets {
		self.process_ticket(ticket);
	    }
	}
    }

    fn process_ticket(&self, ticket: &SupportTicket) {
	println!("========================================");
	println!("Processing ticket id: {}", ticket.id);
	println!("Customer: {}", ticket.customer);
	println!("Issue: {}", ticket.issue);
	println!("========================================");
    }
}

fn main() {
    // create the application
    let mut app = CustomerSupport::new("filo");

    // register a few tickets
    app.create_ticket("John Smith", "My computer makes strange sounds!");
    app.create_ticket("Linus Sebastian", "I can't upload any videos. Please help!");
    app.create_ticket("Arjan Egges", "VSCode doesn't automatically solve my bugs.");

    // process the tickets
    app.process_tickets();
}

