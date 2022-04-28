-- https://leetcode.com/problems/second-highest-salary/

select max(A.salary) as 'SecondHighestSalary'
from Employee A
where A.salary not in  (
    select max(B.salary)
    from Employee B
)