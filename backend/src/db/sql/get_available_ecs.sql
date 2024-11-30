SELECT
    (
        -- ECs earned from uploads
        SELECT
            COALESCE(SUM(pu.ecs_spent * 0.8), 0)
        FROM
            uploads AS up
            JOIN purchases AS pu ON pu.upload_id = up.id
        WHERE
            up.uploader = $ 1
            AND pu.user_id <> up.uploader
    ) + (
        -- ECs given/taken by the system
        SELECT
            COALESCE(SUM(systrans.delta_ec), 0)
        FROM
            system_ec_transactions AS systrans
        WHERE
            systrans.affected_user = $ 1
    ) - (
        -- ECs spent on purchases
        SELECT
            COALESCE(SUM(pu.ecs_spent), 0)
        FROM
            purchases AS pu
        WHERE
            pu.user_id = $ 1
    ) + (
        -- ECs refunded from ratings
        SELECT
            COALESCE(SUM(pu.ecs_spent * 0.2), 0)
        FROM
            purchases AS pu
        WHERE
            pu.user_id = $ 1
            AND pu.rating IS NOT NULL
    ) :: float8 AS ecs_available;
