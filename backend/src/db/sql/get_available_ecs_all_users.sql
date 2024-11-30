WITH ecs_earned_tbl AS (
    -- ECs earned from uploads
    SELECT
        up.uploader AS user_id,
        COALESCE(SUM(pu.ecs_spent * 0.8), 0) AS ecs_earned
    FROM
        uploads AS up
        INNER JOIN purchases AS pu ON pu.upload_id = up.id
    WHERE
        pu.user_id <> up.uploader
    GROUP BY
        up.uploader
),
ecs_spent_tbl AS (
    -- ECs spent on purchases
    SELECT
        pu.user_id,
        COALESCE(SUM(pu.ecs_spent), 0) AS ecs_spent
    FROM
        purchases AS pu
    GROUP BY
        pu.user_id
),
ecs_refunded_tbl AS (
    -- ECs refunded from ratings
    SELECT
        pu.user_id,
        COALESCE(SUM(pu.ecs_spent * 0.2), 0) AS ecs_refunded
    FROM
        purchases AS pu
    WHERE
        pu.rating IS NOT NULL
    GROUP BY
        pu.user_id
),
ecs_system_tbl AS (
    -- ECs given/taken by the system
    SELECT
        systrans.affected_user AS user_id,
        COALESCE(SUM(systrans.delta_ec), 0) AS ecs_system
    FROM
        system_ec_transactions AS systrans
    GROUP BY
        systrans.affected_user
)
SELECT
    u.id AS user_id,
    COALESCE(e.ecs_earned, 0) + COALESCE(s.ecs_system, 0) - COALESCE(sp.ecs_spent, 0) + COALESCE(r.ecs_refunded, 0) AS ecs_available
FROM
    users u
    LEFT JOIN ecs_earned_tbl e ON u.id = e.user_id
    LEFT JOIN ecs_system_tbl s ON u.id = s.user_id
    LEFT JOIN ecs_spent_tbl sp ON u.id = sp.user_id
    LEFT JOIN ecs_refunded_tbl r ON u.id = r.user_id;
