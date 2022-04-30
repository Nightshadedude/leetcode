--https://leetcode.com/problems/department-highest-salary/submissions/

select  D.name as Department
    ,E.name as Employee
    ,E.salary as Salary 
from Employee E
inner join Department D
    on D.id = E.departmentID
left outer join Employee E2
    on E.departmentId = E2.departmentId
    and E2.salary > E.salary
Where E2.salary is null