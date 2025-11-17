import { STAKING_ENDPOINT, SUBSCAN_ENDPOINT } from "@/constants/endpoints";
import { NextResponse } from "next/server";

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const address = searchParams.get("address");

  if (!address) {
    return NextResponse.json(
      {
        error: "Missing 'address' parameter.",
      },
      { status: 400 }
    );
  }
  const url = `${process.env.API_URL}/${STAKING_ENDPOINT}?address=${encodeURIComponent(address)}&page=0&row=10`;
  try {
    const res = await fetch(url);
    const data = await res.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error("Error fetching data from staking endpoint:", error);
    return NextResponse.json(
      {
        error: "Failed to fetch data from staking endpoint.",
      },
      { status: 500 }
    );
  }
}
