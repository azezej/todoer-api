CREATE PROCEDURE SumOfNumbers(IN startNumber INT, IN count INT)
BEGIN
  DECLARE sum INT DEFAULT 0;
  DECLARE i INT DEFAULT 0;

  SET i = startNumber;
  WHILE i < startNumber + count DO
    SET sum = sum + i;
    SET i = i + 1;
  END WHILE;

  SELECT sum AS 'Sum';
END