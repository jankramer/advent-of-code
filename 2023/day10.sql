-- create table input as select * from 'day10.input.csv';
create table input as select * from 'day10.example.csv';

create table grid as
select row_number() over () y,
       unnest(range(1, len(#1)+1)) x,
       unnest(str_split(#1, '')) c
from input;

create function adjacent(x, y, c) as table
select
unnest(case c
  when '|' then [{y: y - 1, x: x}, {y: y + 1, x: x}]
  when '-' then [{y: y, x: x - 1}, {y: y, x: x + 1}]
  when 'L' then [{y: y - 1, x: x}, {y: y, x: x + 1}]
  when 'J' then [{y: y - 1, x: x}, {y: y, x: x - 1}]
  when '7' then [{y: y + 1, x: x}, {y: y, x: x - 1}]
  when 'F' then [{y: y + 1, x: x}, {y: y, x: x + 1}]
  else []
end, recursive := 1) as xs;

create function coords(x,y) as x * 10 + y;

create table grid_lookup as
select grid.x, grid.y, grid.c, adj.x as adj_x, adj.y as adj_y, dest.c as adj_c from grid
left join adjacent(grid.x,grid.y,grid.c) as adj on true
left join grid dest on adj.x = dest.x and adj.y = dest.y;

select * from input;

create table loop as
with recursive steps(x, y, c, i, visited) as (
    select grid_lookup.x,
           grid_lookup.y,
           grid_lookup.c,
           0,
           [coords(x,y)]
      from grid_lookup where c = 'S'
     union all
    select lookup.x,
           lookup.y,
           lookup.c,
           i + 1,
           array_distinct(array_concat(visited, array_agg(coords(adj_x, adj_y)) over ()))
      from steps
      join grid_lookup lookup on lookup.adj_x = steps.x and lookup.adj_y = steps.y
      where i < 10000 and not list_contains(visited, coords(lookup.x, lookup.y))
)
select * from steps;

select max(i) from loop;

with recursive loop2(x, y, c, visited, enclosed, i) as (
    select x, y, c, [], [], 0
      from loop
     where c = 'S'

     union all

    select grid_lookup.x,
           grid_lookup.y,
           grid_lookup.c,
           [],
           [],
           i + 1
      from loop2
      join loop
)

select x,y,c from loop2;
