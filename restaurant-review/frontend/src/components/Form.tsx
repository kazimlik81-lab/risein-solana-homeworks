import { FC } from "react";

interface FormProps {
    title: string;
    description: string;
    rating: number;
    location: string;
    setTitle: (value: string) => void;
    setDescription: (value: string) => void;
    setRating: (value: number) => void;
    setLocation: (value: string) => void;
    handleSubmit: () => void;
    handleUpdate: () => void;
}
const ReviewForm: FC<FormProps> = ({
    title,
    description,
    rating,
    location,
    setTitle,
    setDescription,
    setRating,
    setLocation,
    handleSubmit,
    handleUpdate,
}) => {
    const isValid = () => {
        if (rating < 1 || rating > 10) {
            alert("Rating must be between 1 and 10.");
            return false;
        }
        const textBytes = new TextEncoder().encode(title + description + location).length;
        if (title.trim().length === 0 || location.trim().length === 0 || title.length > 400 || description.length > 400 || location.length > 400 || textBytes + 14 > 1000) {
            alert("Enter a title and location, keep each field under 400 characters, and fit the review within 1,000 bytes.");
            return false;
        }
        return true;
    };
    const formSubmit = (e: any) => {
        e.preventDefault();
        if (!isValid()) return;
        handleSubmit();
    };
    return (
        <form
            className="shadow-md rounded px-8 pt-6 pb-8 mb-4"
            onSubmit={(e) => formSubmit(e)}
        >
            <div className="w-full max-w-xs">
                <div className="mb-4">
                    <label htmlFor="location" className="block text-gray-400 text-sm font-bold mb-2">
                        Location
                    </label>
                    <input
                        className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        id="location"
                        type="text"
                        placeholder="City or area"
                        value={location}
                        onChange={(e) => setLocation(e.target.value)}
                    />
                </div>

                <div className="mb-4">
                    <label className="block text-gray-400 text-sm font-bold mb-2">
                        Title
                    </label>
                    <input
                        className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        id="title"
                        type="text"
                        placeholder="Title"
                        value={title}
                        onChange={(e) => setTitle(e.target.value)}
                    />
                </div>

                <div className="mb-4">
                    <label className="block text-gray-400 text-sm font-bold mb-2">
                        Description
                    </label>
                    <input
                        className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        id="description"
                        type="text"
                        placeholder="Description"
                        value={description}
                        onChange={(e) => setDescription(e.target.value)}
                    />
                </div>

                <div className="mb-4">
                    <label className="block text-gray-400 text-sm font-bold mb-2">
                        Rating
                    </label>
                    <input
                        className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        id="rating"
                        type="number"
                        placeholder="Description"
                        value={rating}
                        onChange={(e) => setRating(Number(e.target.value))}
                        max={10}
                        min={1}
                    />
                </div>

                <div className="flex items-center justify-between">
                        <button type="submit" className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                        Submit
                    </button>
                        <button
                            type="button"
                            onClick={() => { if (isValid()) handleUpdate(); }}
                            className="ml-3 bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                        >
                            Update my review
                        </button>
                </div>
            </div>
        </form>
    );
};

export default ReviewForm;
