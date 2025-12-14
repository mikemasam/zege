import { Button } from "@/components/ui/button";
import UIForm from "@/components/ui/ui-form";
import UIInput from "@/components/ui/ui-input";
import api from "@/lib/api";
import { useNavigate } from "react-router";

const DEFAULT_VALUE = {
  email: "",
  password: "",
};
export default function SignupPage() {
  let navigate = useNavigate();
  const onSubmit = async (form: any) => {
    console.log(form);
    const res = await api.post("/auth/signup", form);
    console.log(res);
    //navigate("/app");
  };
  return (
    <div className="min-h-screen flex flex-col gap-4 items-center justify-center bg-slate-900">
      <div className="w-full max-w-md p-8 bg-slate-800 rounded-2xl shadow-lg border border-slate-700">
        <h1 className="text-3xl font-extrabold text-white text-center mb-6">
          New Account
        </h1>

        <p className="text-center text-gray-400 mb-8">
          Access your event logging dashboard
        </p>

        <UIForm
          className="flex flex-col"
          onSubmit={onSubmit}
          defaultValues={DEFAULT_VALUE}
        >
          <UIInput
            label="Name"
            name="name"
            type="text"
            placeholder="Full Name"
            className="h-auto px-4 py-3 rounded-lg bg-slate-700 border border-slate-600 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-sky-500 focus:border-sky-500"
          />
          <UIInput
            label="Email"
            name="email"
            type="email"
            placeholder="you@example.com"
            className="h-auto px-4 py-3 rounded-lg bg-slate-700 border border-slate-600 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-sky-500 focus:border-sky-500"
          />
          <UIInput
            label="Password"
            name="password"
            type="password"
            placeholder="********"
            className="h-auto px-4 py-3 rounded-lg bg-slate-700 border border-slate-600 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-sky-500 focus:border-sky-500"
          />
          <Button
            type="submit"
            size="lg"
            className="mt-7 bg-gradient-to-r from-sky-500 to-cyan-400 text-black hover:from-sky-400 hover:to-cyan-300 "
          >
            Create Account
          </Button>
        </UIForm>

        <div className="mt-10 flex justify-between items-center text-sm text-gray-400">
          <a href="#" className="hover:text-white transition">
            Help?
          </a>
          <a
            href="https://github.com/mikemasam/zege"
            target="_blank"
            className="hover:text-white transition"
          >
            Star on GitHub
          </a>
        </div>
      </div>
      <div className="mt-6 text-center text-sm text-gray-400">
        Already have an account?{" "}
        <a
          href="/login"
          className="font-semibold text-sky-400 hover:text-sky-300 transition"
        >
          Login
        </a>
      </div>
    </div>
  );
}
