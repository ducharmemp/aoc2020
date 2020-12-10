package com.ducharmemp.eight;

import java.util.Vector;
import java.util.stream.Collectors;

public class ProgramSimulator {
  public int endOfProgram;
  public Vector<Instruction> lines;

  public int programCounter = 0;
  public int accumultator = 0;

  public ProgramSimulator(Vector<String> lines) {
    this.lines = lines.stream().map(line -> new Instruction(line)).collect(Collectors.toCollection(Vector::new));
  }

  public Instruction step() {
    Instruction currentInstruction = this.lines.get(this.programCounter);
    Instruction nextInstruction;
    try {
      nextInstruction = this.lines.get(this.programCounter + 1);
    } catch (ArrayIndexOutOfBoundsException e) {
      nextInstruction = null;
    }

    ProgramStep currentStep = new ProgramStep(0, 0);
    switch (currentInstruction.command) {
      case ACC:
        currentStep.accumultator = this.handleAccumulator(currentInstruction);
        currentStep.programCounter = 1;
        break;
      case JMP:
        currentStep.programCounter = this.handleJump(currentInstruction);
        break;
      case NOP:
        currentStep.programCounter = 1;
        break;
    }

    this.accumultator += currentStep.accumultator;
    this.programCounter += currentStep.programCounter;

    return nextInstruction;
  }

  public ProgramSimulator cloneSwappedInstruction() {
    ProgramSimulator ps = new ProgramSimulator(new Vector<>());
    Vector<Instruction> lines = new Vector<>(this.lines);
    Instruction currentInstruction = this.lines.get(this.programCounter);
    lines.set(this.programCounter, currentInstruction.swapped());
    ps.lines = lines;
    return ps;
  }

  public boolean terminated() {
    return this.programCounter >= this.lines.size();
  }

  private int handleAccumulator(Instruction instruction) {
    switch (instruction.operator) {
      case INCR:
        return instruction.value;
      case DECR:
        return -1 * instruction.value;
      default:
        return instruction.value;
    }
  }

  private int handleJump(Instruction instruction) {
    switch (instruction.operator) {
      case INCR:
        return instruction.value;
      case DECR:
        return -1 * instruction.value;
      default:
        return instruction.value;
    }
  }
}