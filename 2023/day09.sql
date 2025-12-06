create table input as (
  select row_number() over () id,
         [cast(x as int) for x in regexp_extract_all(#1, '[\-\d]+')] as values
   from 'day09.input.csv'
);

create table seq as (
    with recursive deltas(id, values, i) as (
        select id, values, 1
          from input
         union all
        select id,
               [values[x+1] - values[x] for x in range(1, len(values))] as new_deltas,
               i + 1
          from deltas
          where i < 10000 and len(list_filter(new_deltas, x -> x != 0))
    )

    select * from deltas
);

with next_values as (select list_aggr(array_agg(values[-1]), 'sum') x from seq group by id)
select sum(x) as part_a from next_values;

with recursive prev(id, value, xs) as (
    select id, 0, list_reverse(array_agg(values[1])) as xs
      from seq
  group by id
    union all
    select id, xs[1] - value, array_pop_front(xs)
     from prev
     where len(xs) > 0
)

select sum(value) as part_b from prev where len(xs) = 0;
