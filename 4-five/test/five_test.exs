defmodule FiveTest do
  use ExUnit.Case
  doctest Five

  test "greets the world" do
    assert Five.hello() == :world
  end
end
