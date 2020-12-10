package com.ducharmemp.eight;

public class Instruction {
    InstructionCommand command;
    InstructionOperator operator;
    int value;

    public enum InstructionCommand {
      JMP,
      ACC,
      NOP
    }

    public enum InstructionOperator {
      INCR, DECR
    }

    Instruction(String instruction) {
      String[] parts = instruction.split("\\s+");
      int value = Integer.parseInt(parts[1].replaceAll("[+-]", ""));

      InstructionCommand command;
      if (parts[0].equalsIgnoreCase("jmp")) {
        command = InstructionCommand.JMP;
      } else if (parts[0].equalsIgnoreCase("acc")) {
        command = InstructionCommand.ACC;
      } else {
        command = InstructionCommand.NOP;
      }

      InstructionOperator operator;
      if (parts[1].startsWith("+")) {
        operator = InstructionOperator.INCR;
      } else {
        operator = InstructionOperator.DECR;
      }

      this.command = command;
      this.operator = operator;
      this.value = value;
    }

    public Instruction(Instruction instruction) {
      this.command = instruction.command;
      this.operator = instruction.operator;
      this.value = instruction.value;
    }

    public Instruction swapped() {
      Instruction newInstruction = new Instruction(this);
      if (this.command == InstructionCommand.JMP) {
        newInstruction.command = InstructionCommand.NOP;
      } else if (this.command == InstructionCommand.NOP) {
        newInstruction.command = InstructionCommand.JMP;
      }

      return newInstruction;
    }

    public String toString() {
      return "command: " + this.command + ", operator: " + this.operator + ", val: " + this.value;
    }
  }