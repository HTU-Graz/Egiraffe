WITH -- ECS earned from uploads
ecs_earned_tbl AS (
    SELECT
        COALESCE(SUM(pu.ecs_spent * 0.8), 0) AS ecs_earned
    FROM
        upload AS up
        INNER JOIN purchase AS pu ON pu.upload_id = up.id
    WHERE
        up.uploader = $1
        AND pu.user_id <> up.uploader -- exclude self-purchases
),
-- ECS spent on purchases
ecs_spent_tbl AS (
    SELECT
        COALESCE(SUM(pu.ecs_spent), 0) AS ecs_spent
    FROM
        purchase AS pu
    WHERE
        pu.user_id = $1
),
-- ECS given/taken by the system
ecs_system_tbl AS (
    SELECT
        COALESCE(SUM(systrans.delta_ec), 0) AS ecs_system
    FROM
        system_ec_transaction AS systrans
    WHERE
        systrans.affected_user = $1
)
SELECT
    ecs_earned + ecs_system - ecs_spent AS ecs_available
FROM
    ecs_earned_tbl
    CROSS JOIN ecs_spent_tbl
    CROSS JOIN ecs_system_tbl;
