-- Calculates the amount of ECS the user has available to spend
--
-- Takes into account:
--
-- - ECS the user has earned (when a purchased upload is owned by the user)
-- - When a user has spent ECS on a purchase
-- - When the system has given/taken ECS from the user (see table `system_ec_transaction`)
-- ECS earned from uploads
WITH -- 
ecs_earned_tbl AS (
    SELECT
        SUM(pu.ecs_spent * 0.8) AS ecs_earned
    FROM
        upload AS up
        INNER JOIN purchase AS pu ON pu.upload_id = up.id
        INNER JOIN "user" AS us ON us.id = up.uploader
    WHERE
        up.uploader = $1
        AND pu.user_id <> up.uploader -- exclude self-purchases
    GROUP BY
        up.uploader
),
-- ECS spent on purchases
ecs_spent_tbl AS (
    SELECT
        SUM(pu.ecs_spent) AS ecs_spent
    FROM
        purchase AS pu
    WHERE
        pu.user_id = $1
    GROUP BY
        pu.user_id
),
-- ECS given/taken by the system
ecs_system_tbl AS (
    SELECT
        SUM(systrans.delta_ec) AS ecs_system
    FROM
        system_ec_transaction AS systrans
    WHERE
        systrans.affected_user = $1
    GROUP BY
        systrans.affected_user
)
SELECT
    -- cast this to anything that's not numeric, but has the largest possible length
    FOR reference: ecs_earned + ecs_system - ecs_spent AS ecs_available
FROM
    ecs_earned_tbl
    CROSS JOIN ecs_spent_tbl
    CROSS JOIN ecs_system_tbl;
