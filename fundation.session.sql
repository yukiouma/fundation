-- -- max nav in latest one month
-- CREATE VIEW `latest_1_month_nav` AS
-- SELECT `fund`.`id`,
--     `fund`.`code`,
--     `fund`.`name`,
--     `nav`.`max_nav`,
--     `nav`.`min_nav`
-- FROM `fund` AS fund
--     LEFT JOIN (
--         SELECT `fund_id`,
--             MAX(`nav`) AS max_nav,
--             MIN(`nav`) AS min_nav
--         FROM `fund_nav`
--         WHERE `date` >= DATE_SUB(CURDATE(), INTERVAL 6 MONTH)
--         GROUP BY `fund_id`
--     ) as nav on `fund`.`id` = `nav`.`fund_id`;
-- TRUNCATE TABLE fund;
-- TRUNCATE TABLE fund_nav;
-- SELECT *
-- FROM `fund_nav`
-- WHERE `fund_id` = 2
-- ORDER BY date DESC;
-- DROP VIEW latest_1_month_nav;
-- DROP VIEW latest_3_month_nav;
-- DROP VIEW latest_12_month_nav;
SELECT *
FROM fund_nav
WHERE fund_id = 3
ORDER BY date DESC;
-- UPDATE `fund`
-- SET `name` = ""
-- WHERE `id` = 1;