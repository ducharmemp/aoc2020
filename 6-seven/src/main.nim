import strutils
import tables
import sequtils, sugar
import re
import sets

type
  RelationshipType = enum
    Parent, Child

type
    Graph* = object
      neighbors*: Table[string, seq[(int, RelationshipType, string)]]

proc insertEntry*(g: var Graph, entry: string) =
  if entry notin g.neighbors:
    g.neighbors[entry] = @[]

proc insertNeighbor*(g: var Graph, parent: string, neighbor: string, weight: int = 0) =
  g.insertEntry(neighbor)
  g.neighbors[parent].add((weight, RelationshipType.Parent, neighbor))
  g.neighbors[neighbor].add((weight, RelationshipType.Child, parent))

proc walkFrom*(g: Graph, entry: string, typ: RelationshipType = RelationshipType.Child): seq[string] =
  var stack: seq[string] = @[entry]
  var path: seq[string] = @[]
  while stack.len != 0:
    let tail = pop(stack)
    if tail notin path and tail != entry:
      path.add(tail)

    for neighbor in g.neighbors[tail].filter(neighbor => neighbor[1] == typ):
      if neighbor[2] in path:
        continue
      stack.add(neighbor[2])

  path

proc sumFrom*(g: Graph, entry: string, typ: RelationshipType = RelationshipType.Child): int =
  var seen = @[entry]
  
  proc inner(entry: string, typ: RelationshipType, seen: var seq[string]): int =
    var total = 1
    for neighbor in g.neighbors[entry].filter(neighbor => neighbor[1] == typ):
      let innerVal = inner(neighbor[2], typ, seen=seen)
      echo neighbor, " ", total, " ", innerVal
      seen.add(neighbor[2])
      total += neighbor[0] * innerVal

    total

  inner(entry, typ, seen) - 1


proc solution1(lines: seq[string]): string =
  var graph = Graph(neighbors: initTable[string, seq[(int, RelationshipType, string)]]())
  let toFind = "shiny gold"
  for line in lines:
    if line == "":
      continue
    let items = split(line, "contain", 1)
    let parent = strip(items[0].replace(re"[^a-zA-Z ]+|bags?", ""))
    let children = split(items[1], ",").map(child => strip(child.replace(re"[^a-zA-Z ]+|bags?", "")))
    graph.insertEntry(parent)
    for child in children:
      if child == "no other":
        continue
      graph.insertNeighbor(parent, child)
    
  let entries = graph.walkFrom(toFind)
  echo toHashSet(entries).len

proc solution2(lines: seq[string]): string =
  var graph = Graph(neighbors: initTable[string, seq[(int, RelationshipType, string)]]())
  let toFind = "shiny gold"
  for line in lines:
    if line == "":
      continue
    let items = split(line, "contain", 1)
    let parent = strip(items[0].replace(re"[^a-zA-Z ]+|bags?", ""))
    let children = split(items[1], ",").map(child => strip(child.replace(re"[^a-zA-Z0-9 ]+|bags?", ""))).map(child => split(child, " ", 1))
    graph.insertEntry(parent)
    for child in children:
      if child[0] == "no" and child[1] ==  "other":
        continue
      graph.insertNeighbor(parent, child[1], weight=parseInt(child[0]))
    
  # echo graph.neighbors["shiny gold"]
  # echo graph.neighbors["striped beige"]
  let total = graph.sumFrom(toFind, typ = RelationshipType.Parent)
  echo total

proc main(): int =
  let input = readFile("input.txt")
  let lines = split(input, "\n")
  echo solution1(lines)
  echo solution2(lines)
  0

discard main()