// A rust version of run_vectors.sh from RFC6716 (actually, a pretty dumb translation).
// This allows us to run tests on Windows.

// Copyright (c) 2011-2012 Jean-Marc Valin
//
//  This file is extracted from RFC6716. Please see that RFC for additional
//  information.
//
//  Redistribution and use in source and binary forms, with or without
//  modification, are permitted provided that the following conditions
//  are met:
//
//  - Redistributions of source code must retain the above copyright
//  notice, this list of conditions and the following disclaimer.
//
//  - Redistributions in binary form must reproduce the above copyright
//  notice, this list of conditions and the following disclaimer in the
//  documentation and/or other materials provided with the distribution.
//
//  - Neither the name of Internet Society, IETF or IETF Trust, nor the
//  names of specific contributors, may be used to endorse or promote
//  products derived from this software without specific prior written
//  permission.
//
//  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
//  ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
//  LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
//  A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
//  OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
//  EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
//  PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
//  PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
//  LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
//  NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
//  SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() {
    let mut args = std::env::args();
    let argv0 = args.next().unwrap();

    if args.len() != 3 {
        eprintln!("usage: {} <exec path> <vector path> <rate>", argv0);
        std::process::exit(1);
    }

    let exec_path = PathBuf::from(args.next().unwrap());
    let vector_path = PathBuf::from(args.next().unwrap());
    let rate = args.next().unwrap().parse::<u32>().unwrap();

    let exe_suffix = if cfg!(windows) { ".exe" } else { "" };

    let opus_demo = exec_path.join(format!("opus_demo{}", exe_suffix));
    let opus_compare = exec_path.join(format!("opus_compare{}", exe_suffix));

    if vector_path.is_dir() {
        eprintln!("Test vectors found in {:?}", vector_path);
    } else {
        eprintln!("No test vectors found");
        std::process::exit(1);
    }

    if !opus_compare.is_file() {
        eprintln!("ERROR: Compare program not found: {:?}", opus_compare);
        std::process::exit(1);
    }

    if opus_demo.is_file() {
        eprintln!("Decoding with {:?}", opus_demo);
    } else {
        eprintln!("ERROR: Decoder not found: {:?}", opus_demo);
        std::process::exit(1);
    }

    eprintln!("==============");
    eprintln!("Testing mono");
    eprintln!("==============");
    eprintln!();

    let logs_mono = File::create("logs_mono.txt").unwrap();
    let logs_mono2 = File::create("logs_mono2.txt").unwrap();

    for file in [
        "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12",
    ]
    .iter()
    {
        let bitstream = vector_path.join(format!("testvector{}.bit", file));
        if bitstream.is_file() {
            eprintln!("Testing testvector{}", file);
        } else {
            eprintln!("Bitstream file not found: testvector{}", file);
        }
        let mut cmd = std::process::Command::new(&opus_demo);
        cmd.arg("-d")
            .arg(rate.to_string())
            .arg("1")
            .arg(bitstream)
            .arg("tmp.out")
            .stdout(std::process::Stdio::from(logs_mono.try_clone().unwrap()))
            .stderr(std::process::Stdio::from(logs_mono.try_clone().unwrap()));
        if cmd.status().unwrap().success() {
            eprintln!("successfully decoded");
        } else {
            eprintln!("ERROR: decoding failed");
            std::process::exit(1);
        }
        let mut cmd = std::process::Command::new(&opus_compare);
        cmd.arg("-r")
            .arg(rate.to_string())
            .arg(vector_path.join(format!("testvector{}.dec", file)))
            .arg("tmp.out")
            .stdout(std::process::Stdio::from(logs_mono.try_clone().unwrap()))
            .stderr(std::process::Stdio::from(logs_mono.try_clone().unwrap()));
        let float_ret = cmd.status().unwrap().success();
        let mut cmd = std::process::Command::new(&opus_compare);
        cmd.arg("-r")
            .arg(rate.to_string())
            .arg(vector_path.join(format!("testvector{}m.dec", file)))
            .arg("tmp.out")
            .stdout(std::process::Stdio::from(logs_mono2.try_clone().unwrap()))
            .stderr(std::process::Stdio::from(logs_mono2.try_clone().unwrap()));
        let float_ret2 = cmd.status().unwrap().success();
        if float_ret || float_ret2 {
            eprintln!("output matches reference");
        } else {
            eprintln!("ERROR: output does not match reference");
            std::process::exit(1);
        }
        eprintln!();
    }
    drop(logs_mono);
    drop(logs_mono2);

    eprintln!("==============");
    eprintln!("Testing stereo");
    eprintln!("==============");
    eprintln!();

    let logs_stereo = File::create("logs_stereo.txt").unwrap();
    let logs_stereo2 = File::create("logs_stereo2.txt").unwrap();

    for file in [
        "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12",
    ] {
        let bitstream = vector_path.join(format!("testvector{}.bit", file));
        if bitstream.is_file() {
            eprintln!("Testing testvector{}", file);
        } else {
            eprintln!("Bitstream file not found: testvector{}", file);
        }
        let mut cmd = std::process::Command::new(&opus_demo);
        cmd.arg("-d")
            .arg(rate.to_string())
            .arg("2")
            .arg(bitstream)
            .arg("tmp.out")
            .stdout(std::process::Stdio::from(logs_stereo.try_clone().unwrap()))
            .stderr(std::process::Stdio::from(logs_stereo.try_clone().unwrap()));
        if cmd.status().unwrap().success() {
            eprintln!("successfully decoded");
        } else {
            eprintln!("ERROR: decoding failed");
            std::process::exit(1);
        }
        let mut cmd = std::process::Command::new(&opus_compare);
        cmd.arg("-s")
            .arg("-r")
            .arg(rate.to_string())
            .arg(vector_path.join(format!("testvector{}.dec", file)))
            .arg("tmp.out")
            .stdout(std::process::Stdio::from(logs_stereo.try_clone().unwrap()))
            .stderr(std::process::Stdio::from(logs_stereo.try_clone().unwrap()));
        let float_ret = cmd.status().unwrap().success();
        let mut cmd = std::process::Command::new(&opus_compare);
        cmd.arg("-s")
            .arg("-r")
            .arg(rate.to_string())
            .arg(vector_path.join(format!("testvector{}m.dec", file)))
            .arg("tmp.out")
            .stdout(std::process::Stdio::from(logs_stereo2.try_clone().unwrap()))
            .stderr(std::process::Stdio::from(logs_stereo2.try_clone().unwrap()));
        let float_ret2 = cmd.status().unwrap().success();
        if float_ret || float_ret2 {
            eprintln!("output matches reference");
        } else {
            eprintln!("ERROR: output does not match reference");
            std::process::exit(1);
        }
        eprintln!();
    }

    drop(logs_stereo);
    drop(logs_stereo2);

    eprintln!("All tests have passed successfully");
    let mono1 = BufReader::new(File::open("logs_mono.txt").unwrap())
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| l.contains("quality"))
        .map(|l| l.split_whitespace().nth(3).unwrap().parse::<f32>().unwrap())
        .collect::<Vec<_>>();
    let mono1 = if mono1.len() == 12 {
        mono1.iter().sum::<f32>() / 12.0
    } else {
        0.0
    };
    let mono2 = BufReader::new(File::open("logs_mono2.txt").unwrap())
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| l.contains("quality"))
        .map(|l| l.split_whitespace().nth(3).unwrap().parse::<f32>().unwrap())
        .collect::<Vec<_>>();
    let mono2 = if mono2.len() == 12 {
        mono2.iter().sum::<f32>() / 12.0
    } else {
        0.0
    };

    let mono = if mono2 > mono1 { mono2 } else { mono1 };
    eprintln!("Average mono quality is {}%", mono);

    let stereo1 = BufReader::new(File::open("logs_stereo.txt").unwrap())
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| l.contains("quality"))
        .map(|l| l.split_whitespace().nth(3).unwrap().parse::<f32>().unwrap())
        .collect::<Vec<_>>();
    let stereo1 = if stereo1.len() == 12 {
        stereo1.iter().sum::<f32>() / 12.0
    } else {
        0.0
    };
    let stereo2 = BufReader::new(File::open("logs_stereo2.txt").unwrap())
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| l.contains("quality"))
        .map(|l| l.split_whitespace().nth(3).unwrap().parse::<f32>().unwrap())
        .collect::<Vec<_>>();
    let stereo2 = if stereo2.len() == 12 {
        stereo2.iter().sum::<f32>() / 12.0
    } else {
        0.0
    };

    let stereo = if stereo2 > stereo1 { stereo2 } else { stereo1 };
    eprintln!("Average stereo quality is {}%", stereo);
}
