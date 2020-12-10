package com.ducharmemp.eight;

import java.util.HashSet;

public class DuplicateInstructionWatcher {
  private ProgramSimulator simulator;

  private HashSet<Integer> seen = new HashSet<Integer>();

  public DuplicateInstructionWatcher(ProgramSimulator simulator) {
    this.simulator = simulator;
  }

  public void watch() {
    while (!this.simulator.terminated()) {
      this.simulator.step();

      if (this.seen.contains(this.simulator.programCounter)) {
        break;
      } else {
        this.seen.add(this.simulator.programCounter);
      }
    }
  }

  public ProgramSimulator inner() {
    return this.simulator;
  }
}
