# https://leetcode.com/problems/two-sum/

def twoSum(nums, target):
    """
    :type nums: List[int]
    :type target: int
    :rtype: List[int]
    """
    found = []
    while len(found) == 0:
        last = (len(nums)-1, nums.pop())
        for e,x in enumerate(nums):
            if last[1] + x == target:
                found.append(e)
                found.append(last[0])
    return found

test = [2,7,11,15]
print(twoSum(test, 9))    