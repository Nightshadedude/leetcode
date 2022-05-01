-- https://leetcode.com/problems/department-top-three-salaries/submissions/

select D.name as Department
    ,E2.name as Employee
    ,E2.salary as Salary
from (
    select 
    ROW_NUMBER() OVER(PARTITION BY U.departmentID
           ORDER BY U.salary DESC) AS row_num
           ,U.salary
           ,U.departmentID

    from (
        select distinct E.salary, E.departmentID
        from employee E
    ) as U
) as RS
left join employee E2
    on E2.salary = RS.salary
    and E2.departmentID = RS.departmentID
left join department D
    on E2.departmentID = D.id
where RS.row_num between 1 and 3

;