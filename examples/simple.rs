use armagetronapi as tron;

fn main() {
    // First, set up all the data you need to persist between each laggerlog entry
    // Consider setting up a struct to keep them all bundled nicely together!
    let mut my_persistent_data: u8 = 0;

    // Second, define a function that handles each incoming ladderlog entry
    // (see below)

    // Finally, call armagetronapi::run with a closure that takes a LadderLogEntry
    tron::runtime::run(|entry| process(entry, &mut my_persistent_data));
}

// Pass your persistent data in as mutable references.
// This prevents the need for statics.
fn process(entry: tron::model::ladderlog::LadderLogEntry, persistent_data: &mut u8) {
    // This function will be called once for each incoming ladder log from armagetron

    // First, match on the entry!
    match entry {
        tron::model::ladderlog::LadderLogEntry::Chat(_, _, _) => {
            // Do what you need to do.
            *persistent_data += 1;
        }
        _ => (),
    }
}
