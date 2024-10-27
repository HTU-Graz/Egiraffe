import { useRouteData } from "@solidjs/router";
import { createSignal, Show } from "solid-js";
import { AscendingIcon, DescendingIcon } from "../../icons/Sorting";

/**
 * @file This page displays and manages all users in the system. It is only accessible to administrators.
 */
export default function Users() {
    // FIXME This is a placeholder. Implement the actual functionality.
    // const users = useRouteData<UploadsDataType>();
    const [activeSort, setActiveSort] = createSignal("date");
    const [sortDateDirection, setSortDateDirection] = createSignal(false);
    const [sortSizeDirection, setSortSizeDirection] = createSignal(false);
    const [sortDownloadsDirection, setSortDownloadsDirection] = createSignal(false);
    const [sortRatingDirection, setSortRatingDirection] = createSignal(false);

    return (
        <>
            <p>todo</p>
        </>
    );
}
