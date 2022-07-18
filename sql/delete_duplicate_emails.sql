--https://leetcode.com/problems/delete-duplicate-emails/submissions/

delete from person where id in(
    select p1.id
    from person p1
    inner join person p2
        on p2.email = p1.email
    where p1.id > p2.id
)