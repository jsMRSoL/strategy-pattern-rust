#![allow(dead_code)]
mod random;

use rand::seq::SliceRandom;
use rand::thread_rng;
use random::Random;

fn generate_id(length: u8) -> String {
    Random::rand_alpha(length)
}

#[derive(Debug)]
struct SupportTicket {
    id: String,
    customer: String,
    issue: String,
}

impl Clone for SupportTicket {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            customer: self.customer.clone(),
            issue: self.issue.clone(),
        }
    }
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

trait TicketOrdering {
    fn create_ordering(&self, list: &mut Vec<SupportTicket>);
}

struct FIFOOrderingStrategy();
struct FILOOrderingStrategy();
struct RandomOrderingStrategy();

impl TicketOrdering for FIFOOrderingStrategy {
    fn create_ordering(&self, _list: &mut Vec<SupportTicket>) {}
}

impl TicketOrdering for FILOOrderingStrategy {
    fn create_ordering(&self, list: &mut Vec<SupportTicket>) {
        list.reverse();
    }
}

impl TicketOrdering for RandomOrderingStrategy {
    fn create_ordering(&self, list: &mut Vec<SupportTicket>) {
        list.shuffle(&mut thread_rng());
    }
}

struct CustomerSupport<T> {
    tickets: Vec<SupportTicket>,
    processing_strategy: T,
}

impl<T: TicketOrdering> CustomerSupport<T> {
    fn new(processing_strategy: T) -> Self {
        Self {
            tickets: Vec::new(),
            processing_strategy,
        }
    }

    fn create_ticket(&mut self, customer: &str, issue: &str) {
        self.tickets.push(SupportTicket::new(customer, issue));
    }

    fn process_tickets(&mut self) {
	// let ticket_list = &mut self.tickets;
	self.processing_strategy.create_ordering(&mut self.tickets);
        if self.tickets.len() == 0 {
            println!("There are no tickets to process. Well done!");
            return;
        }

        for ticket in &self.tickets {
            self.process_ticket(&ticket);
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
    let mut app = CustomerSupport::new(RandomOrderingStrategy());

    // register a few tickets
    app.create_ticket("John Smith", "My computer makes strange sounds!");
    app.create_ticket("Linus Sebastian", "I can't upload any videos. Please help!");
    app.create_ticket("Arjan Egges", "VSCode doesn't automatically solve my bugs.");

    // process the tickets
    app.process_tickets();
}
