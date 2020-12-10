package com.ducharmemp.eight;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.Vector;
import java.util.stream.Collectors;

public class Eight {
  private Vector<String> lines;

  public Eight(String filename) throws IOException {
    this.lines = readLines(filename);
    
  }

  public void processSolution1() {
    DuplicateInstructionWatcher di = new DuplicateInstructionWatcher(new ProgramSimulator(this.lines));

    di.watch();
    System.out.println(di.inner().accumultator);
  }

  public void processSolution2() throws IOException {
    ProgramSimulator ps = new ProgramSimulator(this.lines);

    while (true) {
      Instruction instruction = ps.step();
      if (instruction.command == Instruction.InstructionCommand.JMP || instruction.command == Instruction.InstructionCommand.NOP) {
        DuplicateInstructionWatcher di = new DuplicateInstructionWatcher(ps.cloneSwappedInstruction());

        di.watch();
        
        if (di.inner().terminated()) {
          System.out.println(di.inner().accumultator);
          break;
        }
      }
    }
  }

  public static Vector<String> readLines(String filename) throws IOException {
    BufferedReader br = new BufferedReader(new FileReader(filename));
    try {
      return br.lines().collect(Collectors.toCollection(Vector::new));
    } finally {
      br.close();
    }
  }

  public static void main(String[] args) throws IOException {
    Eight eight = new Eight(args[0]);
    eight.processSolution1();
    eight.processSolution2();
  }
}