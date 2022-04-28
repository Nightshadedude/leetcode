-- https://leetcode.com/problems/duplicate-emails/submissions/

select distinct email as Email
from Person
group by email
having count(email) > 1