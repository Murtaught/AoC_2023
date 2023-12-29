from pos import Pos
from enum import Enum

Dir = Enum('Dir', list('NESW'))

def dir_reverse(d):
    match d:
        case Dir.N: return Dir.S
        case Dir.E: return Dir.W
        case Dir.S: return Dir.N
        case Dir.W: return Dir.E

def go(p, d):
    match d:
        case Dir.N: return Pos(p.i - 1, p.j)
        case Dir.E: return Pos(p.i, p.j + 1)
        case Dir.S: return Pos(p.i + 1, p.j)
        case Dir.W: return Pos(p.i, p.j - 1)

