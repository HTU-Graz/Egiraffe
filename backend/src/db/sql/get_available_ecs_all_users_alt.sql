SELECT
    u.id AS user_id,
    (
        -- ECs earned from uploads
        SELECT
            COALESCE(SUM(pu.ecs_spent * 0.8), 0)
        FROM
            upload AS up
            JOIN purchase AS pu ON pu.upload_id = up.id
        WHERE
            up.uploader = u.id
            AND pu.user_id <> up.uploader
    ) + (
        -- ECs given/taken by the system
        SELECT
            COALESCE(SUM(systrans.delta_ec), 0)
        FROM
            system_ec_transaction AS systrans
        WHERE
            systrans.affected_user = u.id
    ) - (
        -- ECs spent on purchases
        SELECT
            COALESCE(SUM(pu.ecs_spent), 0)
        FROM
            purchase AS pu
        WHERE
            pu.user_id = u.id
    ) + (
        -- ECs refunded from ratings
        SELECT
            COALESCE(SUM(pu.ecs_spent * 0.2), 0)
        FROM
            purchase AS pu
        WHERE
            pu.user_id = u.id
            AND pu.rating IS NOT NULL
    ) AS ecs_available
FROM
    users AS u;
