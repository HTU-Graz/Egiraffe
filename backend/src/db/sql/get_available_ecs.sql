-- Calculates the amount of ECS the user has available to spend
--
-- Takes into account:
--
-- - ECS the user has earned (when a purchased upload is owned by the user)
-- - When a user has spent ECS on a purchase
-- - When the system has given/taken ECS from the user (see table `system_ec_transaction`)
WITH cte AS (
    -- ECS earned from uploads
    ecs_earned AS (
        SELECT SUM(pu.ecs_spent)
        FROM upload AS up
            INNER JOIN purchase AS pu ON pu.upload_id = up.id
            INNER JOIN "user" AS us ON us.id = up.uploader
        WHERE up.uploader = $1
        GROUP BY up.uploader -- FIXME also consider the case where the purchaser has rated the upload
    ),
    -- ECS spent on purchases
    ecs_spent AS (
        SELECT SUM(pu.ecs_spent)
        FROM purchase AS pu
        WHERE pu.user_id = $1
        GROUP BY pu.user_id
    ),
    -- ECS given/taken by the system
    ecs_system AS (
        SELECT SUM(systrans.amount)
        FROM system_ec_transaction AS systrans
        WHERE systrans.affected_user = $1
        GROUP BY systrans.affected_user
    )
)
SELECT ecs_earned + ecs_system - ecs_spent AS ecs_available
FROM cte
