--https://leetcode.com/problems/customers-who-never-order/submissions/

select cust.name as Customers
from Customers cust
left join Orders ord
    on ord.customerId = cust.id
where ord.id is null