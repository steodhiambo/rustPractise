#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ErrorOffice {
    OfficeClose(u32),
    OfficeNotFound(u32),
    OfficeFull(u32),
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OfficeOne {
    pub next_office: Result<OfficeTwo, ErrorOffice>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OfficeTwo {
    pub next_office: Result<OfficeThree, ErrorOffice>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OfficeThree {
    pub next_office: Result<OfficeFour, ErrorOffice>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OfficeFour {
    pub document_id: Result<u32, ErrorOffice>,
}

impl OfficeOne {
    pub fn get_document_id(&self) -> Result<u32, ErrorOffice> {
        // Chain through all offices using the ? operator
        // which will return early if any Result is Err
        let office_two = self.next_office?;
        let office_three = office_two.next_office?;
        let office_four = office_three.next_office?;
        office_four.document_id
    }
}

fn main() {
    let office_ok = OfficeOne {
        next_office: Ok(OfficeTwo {
            next_office: Ok(OfficeThree {
                next_office: Ok(OfficeFour {
                    document_id: Ok(13),
                }),
            }),
        }),
    };
    let office_closed = {
        OfficeOne {
            next_office: Ok(OfficeTwo {
                next_office: Err(ErrorOffice::OfficeClose(23)),
            }),
        }
    };

    match office_ok.get_document_id() {
        Ok(id) => println!("Found a document with id {}", id),
        Err(err) => println!("Error: {:?}", err),
    };
    match office_closed.get_document_id() {
        Ok(id) => println!("Found a document with id {}", id),
        Err(err) => println!("Error: {:?}", err),
    };
}
// And its output:

// $ cargo run
// Found a document with id 13
// Error: OfficeClose(23)
