use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time;

use threadpool::ThreadPool;

fn main() {
    let source = PathBuf::from("./test_source");
    let target = PathBuf::from("./test_target");

    monitoring(source, target);
}

fn monitoring(source: PathBuf, target: PathBuf) {
    let mut entries = fs::read_dir(&source)
        .expect("Error reading source directory")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
    println!("old: {:?}", entries);

    let pool = ThreadPool::new(8);
    loop {
        let new_entries = fs::read_dir(&source)
            .expect("Error reading target directory")
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap();

        if !new_entries.is_empty() {
            let new_files = new_entries.iter().filter(|path| !entries.contains(&path));

            for file in new_files {
                let file = file.clone();
                let name = target.as_path().join(file.file_name().unwrap());
                pool.execute(move || {
                    println!("Copying {:?} to {:?}", &file, &name);
                    let now = time::Instant::now();

                    match fs::copy(&file, &name) {
                        Ok(_) => (),
                        Err(e) => {
                            panic!("{:?}\nsource:{:?}\ntarget:{:?}", e, &file, &name);
                        }
                    }
                    println!(
                        "Finished copying {:?} in {} secs",
                        file.file_name().unwrap(),
                        now.elapsed().as_secs()
                    );
                })
            }

            entries = new_entries;
        }

        thread::sleep(time::Duration::from_millis(10000));
    }
}
