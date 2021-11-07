use hex;
use regex::Regex;
use sha2::{Digest, Sha512};
use std::fs;
use std::io::{self, BufRead, Write};

fn format_answer(answer: &str, replacer: &Regex) -> String {
	let replaced = replacer.replace_all(answer.trim(), "=");
	let lower = replaced.to_lowercase();
	let mut data = lower.split_whitespace().into_iter().collect::<Vec<&str>>();

	data.sort();

	data.join(" ")
}

fn main() {
	fs::remove_file("script.sql").ok();
	fs::remove_file("script.sh").ok();

	let mut sql = fs::OpenOptions::new()
		.write(true)
		.append(true)
		.create_new(true)
		.open("script.sql")
		.unwrap();

	let mut bash = fs::OpenOptions::new()
		.write(true)
		.append(true)
		.create_new(true)
		.open("script.sh")
		.unwrap();

	bash.write_all(b"#!/bin/bash\n").ok();

	let stdin = io::stdin();
	let replacer = Regex::new("\\s*=\\s*").unwrap();
	let mut hasher = Sha512::new();

	for line in stdin.lock().lines() {
		if let Ok(unwrapped) = line {
			let mut entries = unwrapped.splitn(5, ",,");
			let file = entries.next().unwrap();
			let formatted = format!(
				"{}{}",
				format_answer(entries.next().unwrap(), &replacer),
				file
			);

			hasher.update(&formatted);

			let hash = hex::encode(hasher.finalize_reset());

			hasher.update(format!("{}{}", hash, formatted));

			let flag = hex::encode(hasher.finalize_reset());

			hasher.update(file);

			let file_hash = hex::encode(hasher.finalize_reset());

			sql.write_all(
				format!(
					"INSERT INTO flags VALUES('{}', '{}', {}, '{}', {});\n",
					flag,
					entries.next().unwrap().replace('\'', "''"),
					entries.next().unwrap(),
					entries.next().unwrap(),
					if entries.next().unwrap() == "1" { true } else { false }
				)
				.as_bytes(),
			)
			.ok();

			bash.write_all(format!(
        "touch \"/opt/CyberPatriot/hashes/{}\";\nif [ -s \"{}\" ] then cp \"{}\" \"/opt/CyberPatriot/diffs/{}\" else touch \"/opt/CyberPatriot/diffs/{}\";\necho \"{}\" >> /opt/CyberPatriot/files.dat;\n",
        hash, file, file, file_hash, file_hash, file
      ).as_bytes()).ok();
		}
	}
}